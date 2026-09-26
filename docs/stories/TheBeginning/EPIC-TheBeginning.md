# The beginning - Our first Epic

## Goal

To get the project going, this first Epic is not about the simulation OS itself, it is about having something (written in Rust) that can be used as the simulation to run, the choice of topic is 2D computer graphics and the specific deliverable is a lightweight 2D physics engine that can simulate a world of "balls", and a lightweight wrapper that renders such world in a tick-by-tick simulation that displays how the world behaves starting from some initial conditions.

## Non-Goals

* To actually implement any mechanism to distribute the simulations
* Fully-featured physics simulation
* Implementation of rendering logic
* Any fancy visuals

## Acceptance Criteria

**AC1: The physics engine**

The following concepts are supported:

* A rectangle with width and height containing entities we'll call the world map, or just World for short
* A Ball with diameter and position (x, y)
* The World contains a collection of Balls (obviously with their positions)
* The World tracks time passing in the form of "ticks", the physics engine has a single feature, it can take World at any tick "n" and can produce the World at the next tick "n + 1", given some initial conditions, i.e. the World at tick 0

**AC2: The rendering wrapper**

The following behaviour is supported:

* Given a World at any tick, the state of the world is represented accurately on the screen using a single fixed scaling
* Time progresses linearly one tick at a time at a fixed rate configurable via CLI argument, nothing dynamic just yet
* The rendering can be started, paused and restarted
