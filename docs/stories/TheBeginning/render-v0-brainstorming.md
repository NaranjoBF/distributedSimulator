## The problem

We need to select an approach to graphics for V0, there are many choices, from terminal-centric libraries, lightweight graphics ones with full freedom to work on the pixels themselves, full blown game engines with everything from asset management to 3D rendering, etc.

My requirements would be the following:

* Freedom to control the simulation and rendering loop in my own terms without a lot of imposition from the framework
* Drawing capabilities for 2D rendering decent but not heavyweight
* Compatibility with my distributed architecture requirements

That last one is interesting because I'm writing the physics engine using fixed point arithmetic, I'm wondering if that constrains the rendering side too, I mean, I would have to do some unit transformations, let's say my world uses standard scientific units for the main quantities, seconds for time, meters for length, Kg for mass

For rendering I have at least 2 moving parts, the size of the viewport, which I guess for simplicity purposes can begin fixed, and the scaling factor, i.e. a pixel to meter ratio.

Given I have to make this transformation, and the physics units are fixed point arithmetic I think I need fixed point arithmetic support for the rendering scaling calculations, they're basic: rendered_length = scale_factor * physics_length, however, the display can't possibly have fractional quantities of pixels, they must be integer, that leaves me with 2 more choices, floating/fixed point scaling factor, vs 2 integers, numerator and denominator, with the latter we can probably just use integer operations, but I'm not sure if I still need to deal with fixed point math or I can just use the integer representation and adjust the scaling.

I'm reconsidering, why do I need full freedom over the loop? the core of this project is the distributed simulation, not the rendering, I could learn lots of interesting things from building rendering stuff too, but that would eat effort I could spend on really learning the distributed computing part of the problem more deeply. How much would any of the available libraries get in the way if the aim is to distribute the physics? I guess the boundary is the same from that perspective regardless of the rendering library/framework choice because the distributed version is basically just an extra layer of routing and load balancing or something like that (likely oversimplifying)
