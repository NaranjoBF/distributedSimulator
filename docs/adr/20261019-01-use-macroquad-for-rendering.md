---
id: ADR-20261019-01
title: Use Macroquad for 2D simulation rendering
status: accepted
date: 2026-10-19
topic: architecture
---

# ADR-20261019-01: Use Macroquad for 2D simulation rendering

- **Date**: 2026-10-19
- **Status**: Accepted
- **Related**:
  - [EPIC-TheBeginning.md](../stories/TheBeginning/EPIC-TheBeginning.md)
  - [render-v0-brainstorming.md](../stories/TheBeginning/render-v0-brainstorming.md)
  - [ADR-20260926-01: Use fixed-point arithmetic for physics simulation](20260926-01-use-fixed-point-arithmetic.md)

## Background

The primary goal of the project is to build a distributed simulation engine, with 2D ball physics serving as the initial test workload. To visualize the world tick-by-tick, we require a rendering wrapper that can display balls with fixed scaling, run at a configurable tick rate, and handle basic play/pause/reset controls.

We initially evaluated whether the rendering crate needed to support fixed-point arithmetic (matching the physics engine) and whether we needed full, low-level control over the rendering and simulation loop.

## Decision

Use `macroquad` as the 2D rendering framework for the visualization wrapper, and treat rendering as a pure, one-way floating-point (`f32`) observer of the fixed-point simulation state.

## Rationale

- **Prioritizing the core domain over incidental complexity**: The project's core challenge is distributed computing (spatial partitioning, consensus, network routing, tick synchronization). Writing and maintaining custom windowing, swapchains, event loops, or software circle rasterizers diverts significant effort away from distributed systems problems.
- **Rendering is an ephemeral observer, not part of simulation state**: Determinism and fixed-point math are critical for the simulation engine to stay in sync across nodes, but the renderer is a one-way consumer. Converting fixed-point coordinates (`Scalar`) to `f32` screen space at the render boundary isolates hardware-specific floating-point arithmetic from the simulation state.
- **Natural fit for distributed architecture**: Simulation worker nodes will run headless. The renderer only needs to subscribe to world snapshots (locally in Phase 1 via direct state, and remotely in Phase 2 via network channels/sockets). `macroquad`'s lightweight loop cleanly accommodates polling or receiving snapshots without imposing architectural constraints on the backend.
- **Minimal friction and boilerplate**: `macroquad` provides built-in 2D shape primitives (`draw_circle`) and input handling with near-zero boilerplate, allowing us to satisfy Epic AC2 immediately.

## Rejected alternatives

### Low-level framebuffer (`winit` + `softbuffer` / `pixels`)

- **Pros**: Complete ownership of the application loop; zero framework opinions.
- **Cons**: High incidental complexity. Requires manual window event handling, DPI scaling plumbing, and either custom circle rasterization math or additional 2D rendering dependencies.

### Full game engine (`bevy`)

- **Pros**: Rich 2D graphics, built-in gizmos, and asset pipelines.
- **Cons**: Strongly couples code to Bevy's ECS (Entity Component System) architecture and internal scheduler, conflicting with our standalone domain models and distributed state serialization.

### Immediate mode GUI (`egui` / `eframe`)

- **Pros**: Trivial creation of inspection UI, debug sliders, and controls alongside canvas drawing.
- **Cons**: Useful for debug tooling, but adds extra GUI-specific paradigms when only a lightweight visualization canvas and basic keyboard inputs are needed for V0.
