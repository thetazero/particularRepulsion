use core::f64::consts::PI;
use image::{ColorType, ImageBuffer, Rgb};
use rand::Rng;
use std::cmp::max;
use std::sync::mpsc;
use std::thread;

const THREAD_COUNT: usize = 4;
const CYCLES: usize = 10000;
const PARTICLES_PER_THREAD: usize = 1000;

const WIDTH: isize = 1920 / 2;
const HEIGHT: isize = 1080 / 2;

const UNIT: f64 = 100.0;
const G: f64 = 0.01;
const OBSTACLE_COUNT: usize = 20;

struct Particle {
  x: f64,
  y: f64,
  vx: f64,
  vy: f64,
}

#[derive(Clone, Copy)]
struct Obstacle {
  x: f64,
  y: f64,
}

fn main() {
  let mut obstacles = Vec::with_capacity(OBSTACLE_COUNT);
  for _ in 0..OBSTACLE_COUNT {
    obstacles.push(random_obstacle());
  }

  let mut threads = Vec::new();
  let (tx, rx) = mpsc::channel();
  for _ in 0..THREAD_COUNT {
    let thread_tx = tx.clone();
    let obs = obstacles.clone();
    let handle = thread::spawn(move || {
      let n = (WIDTH * HEIGHT) as usize;
      let mut board = Vec::<usize>::with_capacity(n);
      for _ in 0..n {
        board.push(0);
      }
      for _ in 0..PARTICLES_PER_THREAD {
        simulate_particle(&mut board, &obs);
      }
      thread_tx.send(board).unwrap();
    });
    threads.push(handle);
  }

  let mut board = [0 as usize; (WIDTH * HEIGHT) as usize];
  for _ in 0..THREAD_COUNT {
    let res = rx.recv().unwrap();
    for i in 0..res.len() {
      board[i] += res[i];
    }
  }

  let mut max_val = 0;
  for v in board.iter() {
    max_val = max(*v, max_val);
  }

  let image = ImageBuffer::from_fn(WIDTH as u32, HEIGHT as u32, |x, y| {
    let i = x + y * (WIDTH as u32);
    let c = board[i as usize];
    if c > 0 {
      let intensity = c as f64 / max_val as f64;
      let mut r = intensity.powf(0.5);
      r *= 256.0;
      let mut g = (intensity - 0.5) % 1.0;
      g = g.powf(4.0);
      g *= 256.0;
      let mut b = (intensity - 0.9) % 1.0;
      b = b.powf(0.5);
      b *= 256.0;
      return Rgb([r as u8, g as u8, b as u8]);
    }
    Rgb([0, 0, 0])
  });
  image::save_buffer(
    "test.png",
    &image,
    WIDTH as u32,
    HEIGHT as u32,
    ColorType::Rgb8,
  )
  .unwrap();

  // println!("Hello, world! {}", rng.gen::<f64>());
}

fn random_particle() -> Particle {
  let mut rng = rand::thread_rng();
  let r = UNIT * rng.gen::<f64>().sqrt() + 1.0;
  let theta = rng.gen::<f64>() * 2.0 * PI;
  let v = (rng.gen::<f64>().sqrt() / 2.0 + 0.5) * 0.2;
  let p = Particle {
    x: r * theta.cos(),
    y: r * theta.sin(),
    vx: -theta.cos() * v,
    vy: -theta.sin() * v,
  };
  p
}

fn get_board_coords(x: f64, y: f64) -> isize {
  let w: isize = ((x + UNIT / 2.0) * (WIDTH as f64) / UNIT) as isize;
  let h: isize = ((y + UNIT / 2.0) * (HEIGHT as f64) / UNIT) as isize;
  if w < 0 || w >= WIDTH || h < 0 || h >= HEIGHT {
    return -1;
  }
  return w + h * (WIDTH as isize);
}

fn norm2(x: f64, y: f64) -> f64 {
  return x.powf(2.0) + y.powf(2.0);
}

fn simulate_particle(board: &mut Vec<usize>, obstacles: &Vec<Obstacle>) {
  let mut p = random_particle();
  for _ in 0..CYCLES {
    if norm2(p.x, p.y) > 9.0 * UNIT * UNIT {
      return;
    }
    p.x += p.vx;
    p.y += p.vy;
    let id = get_board_coords(p.x, p.y);
    for obs in obstacles {
      let mut dx = p.x - obs.x;
      let mut dy = p.y - obs.y;
      let r = (dx.powf(2.0) + dy.powf(2.0)).powf(0.5);
      dx /= r;
      dy /= r;
      p.vx += dx * G / r.powf(2.0);
      p.vy += dy * G / r.powf(2.0);
    }
    if id != -1 {
      let id = id as usize;
      board[id] += 1;
    }
  }
  println!("terminated");
}

fn random_obstacle() -> Obstacle {
  let mut rng = rand::thread_rng();
  let m = max(WIDTH, HEIGHT) as f64;
  let w = (WIDTH as f64) / m * UNIT;
  let h = (HEIGHT as f64) / m * UNIT;
  let x = (rng.gen::<f64>() - 0.5) * w;
  let y = (rng.gen::<f64>() - 0.5) * h;
  Obstacle { x, y }
}
