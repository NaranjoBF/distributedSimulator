---
id: ADR-20260926-01
title: Use fixed-point arithmetic for physics simulation
status: accepted
date: 2026-09-26
topic: architecture
---

# ADR-20260926-01: Use fixed-point arithmetic for physics simulation

- **Date**: 2026-09-26
- **Status**: Accepted
- **Related**:
  - [physics-v0-branstorming.md](../stories/TheBeginning/physics-v0-branstorming.md)

## Background

The physics engine requires deterministic simulation across different platforms and machines. Standard hardware floating-point operations can yield slight differences across CPU architectures, compilers, and operating systems due to variations in FPU implementations and rounding modes, breaking synchronization in a distributed simulator.

## Decision

Use fixed-point arithmetic for state representation and physics calculations.

## Rationale

- **Cross-platform determinism**: Integer arithmetic produces bit-identical results across all target architectures and platforms.
- **Foundation for distributed simulation**: Eliminates simulation drift and state divergence across nodes.
- **Simplicity for initial scope**: Representing basic positions, velocities, and gravity using integer-backed fixed-point values is straightforward for early iterations.

## Rejected alternatives

### Hardware floating-point arithmetic (`f32` / `f64`)

- **Pros**: Fast native hardware support (FPU), simpler standard syntax, large dynamic range.
- **Cons**: Not guaranteed to be bit-level deterministic across different platforms, architectures, and compiler optimization levels.

### Software floating-point emulation (soft-float IEEE-754)

- **Pros**: Preserves floating-point ergonomics while enforcing identical cross-platform results.
- **Cons**: Higher complexity and larger performance overhead compared to fixed-point arithmetic for our initial MVP needs.
