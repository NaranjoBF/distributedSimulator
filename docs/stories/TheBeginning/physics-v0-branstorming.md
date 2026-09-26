## Physics V0

The most basic physics I can have, let's make it just implement a very VERY rudimentary form of gravity, going down with constant speed that is proportional to time

I assume a rudimentary engine could be a basic transformer that takes a world object with a collection of items that live inside it with initial conditions (positions and velocities could be), and then it should have a basic function that allows you to input a time and get the state of the world at that time, if we use, say, seconds, we could have something like:

1) Initialize world at t=0s
2) do world.calculate_state(t=10) this would give us a new world (or it could update it in place) with all items with updated state

This could be fine as a naive implementation but going forward to an arbitrary moment in the future could prove problematic due to performance right? say we just have particles that can bounce off the edges and each other, if I set a time long enough after initial conditions, I would have to somehow calculate the trajectories "frame by frame" so that collisions are detected and direction of movement is updated accordingly, the exact moment 2 objects collide is not necessarily going to fall precisely in whatever my choice of time increment is, because physics operate on continuous space, not on discrete increments, simulations are then lossy I suppose, but no idea how, just so that we don't waste a lot of time re-inventing the wheel (no matter how tempting it might be), I'll look it up

After a shot chat with AI I found that this is indeed a problem and the situation I was worrying about, i.e. where we miss a collision due to fast moving objects and too coarse time resolution, is called "tunneling", we can fix it by using

**Continuous Collision Detection (CCD)**

The gist of the idea is, instead of first updating the position of the objects to their "next tick position" and then checking for collisions, we calculate the time of impact (which can be a fraction of the tick, say 0.23 dt), at that exact moment the system applies a correction of trajectory (chaning the velocity vector), and then, for the remainder of the tick (0.77 dt for the example), it calculates the distance to move the object. This is lossless but compute intensive which motivates a simplification called:

**Discrete Collision Detection (DCD)**

Here we don't bother trying to calculate the exact time of collisions, we just move the objects first and then on every tick we check if they overlap, if they do, we move them to the place where they would be "just touching" and we update their velocity, from here we resume just moving objects every tick, this is lossy and for small fast objects (like bullets), tunneling becomes highly likely

Now, the other problem we face is that on a world with large numbers of objects, detecting collisions for every pair of them is expensive at O(n^2), to improve performance the process is typically divided in 2 phases, a broad one that identifies objects that are "close" to each other (using advanced algorithms like quad trees, spacial hashing, etc), and a narrow one that applies collision detection only to those close objects.

This brings me to my next worry, the passage of time is discrete but we must choose tick frequency correctly from client code so that time passes "at the correct rate", I suppose there are a few ways to do this:

1. We can choose a fixed dt (e.g. 0.1 sec), and we make the client code ask for a "next tick state" periodically with period = dt. This makes frame rate (smoothness) dependent on time resolution, and it can in turn produce 2 outcomes, if we compute next state faster than the refresh period we are forced to wait, if we compute slower we make force the client to slow down
2. We can let the simulation run as fast as possible, no need to wait, but the second problem of being too slow remains

This makes me think we should somehow decouple the simulation frame rate from the rendering "pace", I've seen this in some game performance analysis tools, the same game running on machines with different performance, generally speaking proceed at the same pace (unless performance is way too bad), but the "FPS" the games produce is different between the machines, we definitely don't want our game to become unnaturaly fast just because the computer can calculate too quickly, so I guess what we do is to just add more frames if the machine is more powerful.

I think that idea works, we can let **the simulation/physics** go as fast as possible, and, depending on the actual rate of processing ticks, we choose how many frames we generate, as an example, we could have the problem of rendering a car going in straight line from position 0 to position 10m, if we agree that a good default pace for the game would be to let the car complete that path in 10 seconds, then the desired perceived speed of the car is 1 m/s, on a slow machine that takes 1 second to produce an update, we could produce at max 10 frames, one every meter, the game is awful, we see the car teleport one meter at a time. On the other hand, on a very fast machine, that can execute, say, the same update in 0.01s, we can produce 100 frames in one meter, i.e. 100FPS, much smoother, I guess the ultra choppy 1 frame per meter would be so bad that we just prefer to slow down the game in that situation (and I've seen it, running high end games on a crappy machine produces slowness not awful gigantic steps).

So, how do we actually implement this, if the simulation takes x to run a tick, how do we set the value of dt so that the distance is calculated correctly? is it a matter of measuring time maybe? it could be an approximation, I run one frame, I get actual_time = x, then I set dt = x and everything else falls into place, but that computation would have to occur for every frame because performance is not constant, the computer can be doing all sorts of things making actual tick calculation time oscillate a bit (that last part is my assumption)

Second chat with gemini and I understood how to go about implementing this "world time vs simulation tick rate decoupling", the idea is to use what is called **Fixed timestep with an accummulator** or "fix your timestep" for short, this consists of running the physics at a fixed constant rate, and to measure the time rendering actually takes, this is the pseudocode:

```python
PHYSICS_DT = 0.016  # 60 Hz
accumulator = 0
prev_time = time.time()

while True:
    cur_time = time.time()
    frame_time = curr_time - prev_time
    curr_time = prev_time

    # If machine is too slow, let's slow down the simulation
    if frame_time > 0.25:
        frame_time = 0.25

    accummulator += frame_time

    while accummulator >= PHYSICS_DT:
        prev_state = current_state.clone()
        curr_state = physics.calculate_state(curr_state, PHYSICS_DT)
        accummulator -= PHYSICS_DT

    alpha = accummulator / PHYSICS_DT

    render_interpolated(prev_state, curr_state, alpha)
```

For our first version we'll be deliberately simplistic in this way:

1) The world can only contain balls (perfect circles) with fixed diameter
2) We use DCD
3) We only model gravity, no additional horizontal or vertical forces

The relevant equations would be:

r(t) = x(t)i + y(t)j
v(t) = dx/dt + dy/dt
a(t) = dv/dt = dvx/dt + dvy/dt

For the specific simplified scenario, then:

ax = 0
ay = -9.8 m/s^2

then:

vx = vx0 
vy = vy0 - 9.8t

x = x0 + vx0t
y = y0 + vy0t - 4.9t^2

so, for any ball, we can take whatever initial position and velocity we're given and we can calculate the next position and velocity at the next tick by just replacing the relevant t

This analytical implementation is unnecessarily CPU hungry, a simplification like this can be made:

1) we update the velocity as vy = vy - 9.8t
2) we update the position using the new velocity

Finally, because we eventually want to make this work cross-platform deterministically we have 2 choices, either we make sure we comply with IEEE-754 (using soft floats whatever that is), or we use fixed point arithmetic, we're going to use the latter because it is quite easy anyway for our simple first iteration and would make the whole thing more future proof.

Fixed point arithmetic uses a big integer in which some bits represent the integer part and the rest represent the fractional part, this is totally deterministic but has disadvantages like limited world and object size due to integer overflowing, poor performance because floating point (the alternative) normally leverages FPUs (hardware), and implementation is more complex, our simple MVP doesn't need to worry about that for now though, so, see [ADR-20260926-01](../../adr/20260926-01-use-fixed-point-arithmetic.md)
