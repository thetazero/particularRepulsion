# particularRepulsion
Millions of particles launched at 20 static objects with negative mass creates beautiful patterns. 
Used this as an excuse to learn multithreading in rust (and rust in general).

In particular for creating the image below I used 6 million particles split evenly among 6 threads. 
![rendered image](https://github.com/thetazero/particularRepulsion/blob/main/test.png?raw=true)
The coloring is based of how much time particles spend in each pixel and how fast they were moving there on average. 
Dimensions of the image and more can be configured within the code.
