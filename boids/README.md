# BOIDS

An version of a boids simulation using rust and amethyst.

Where is it currently at:

* Boids that follow the standard rules
* Boids dodge "enemies", the purple circles.

Where did I stop:
* Getting a space partitioning system to work to speed up computations
* Tried getting a Grid/Bucket based partitioner up and running, but ran into
some bugs. I'll need to come back, write some tests, and confirm the iterator
for the partitioner actually works. Performance is also quite bad when using 
the partitioner. 

Not up to writing tests for the TiledSpace yet. And that would need to happen
before I can trust its output. There is just too much going on in the loop to
debug using print statements.