# Distributed Simulator

## What is this about

This is a learning project, it is meant to teach me distributed systems, rust and low-level systems programming by working on a fun and ambitious project iteratively, the concept is rather simple, to create a distributed "simulation operating system" that allows me to run a simulation of any sort (starting with very basic physics) in multiple parallel processes that own parts of the simulation, e.g. for 2D computer graphics the simulation could be simply splitting the viewport into rectangular sections with one process "owning" the physics of each.
