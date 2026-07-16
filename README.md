# Traffic Control Simulation 🚦

A real-time 4-way traffic intersection simulation written in [Rust](https://www.rust-lang.org/) using the [Macroquad](https://macroquad.rs/) library.

> Subject goal: solve the traffic problem of your capital city by designing a traffic control strategy and visualizing it with a simulation.

## Table of Contents

- [Overview](#overview)
- [Controls](#controls)
- [Environment](#environment)
  - [Roads](#1-roads)
  - [Traffic Lights](#2-traffic-lights)
  - [Vehicles](#3-vehicles)
- [Project Structure](#project-structure)
- [Key Constants](#key-constants)
- [Building & Running](#building--running)
- [Compliance with the Subject & Audit](#compliance-with-the-subject--audit)
- [Bonus](#bonus)

## Overview

Two roads cross to form an intersection with one lane per direction. Vehicles spawn at the four entry points of the intersection, follow a chosen route (straight, left, or right), and obey traffic lights managed by a controller that dynamically prioritizes the most congested lane.

```
       │  ↑  │
       │  N  │
───────┼─────┼───────
  W ←  │  ·  │  → E
───────┼─────┼───────
       │  S  │
       │  ↓  │
```

## Controls

| Key | Action |
|-----|--------|
| `↑` (Up) | Spawns a vehicle from the **South**, heading North |
| `↓` (Down) | Spawns a vehicle from the **North**, heading South |
| `→` (Right) | Spawns a vehicle from the **West**, heading East |
| `←` (Left) | Spawns a vehicle from the **East**, heading West |
| `R` | Spawns a vehicle from a random direction |
| `Esc` | Ends the simulation |

Every spawned vehicle is assigned a **random route** (straight / left / right), reflected by its color.

A minimum delay (`SPAWN_COOLDOWN`) is enforced between two spawns on the same direction, and a safety-distance check at the spawn point also prevents creating a vehicle too close to another one — so it is not possible to spam vehicle creation by mashing or holding a key.

## Environment

### 1. Roads

Two roads cross to create an intersection, each with **one lane per direction**. Traffic entering the intersection can select a route: turning left, turning right, or continuing straight.

### 2. Traffic Lights

Lights are positioned where each lane enters the intersection and only have two states: **red** and **green**.

The controller (`TrafficController`) works in phases:

- **Green phase** — only one direction is active at a time; vehicles in that lane may cross. Duration is dynamically adjusted based on the congestion of the lane about to become active.
- **All-red clearing phase** — 1.5 seconds during which every light is red, giving vehicles already inside the intersection time to clear before the next direction turns green.
- **Next lane selection** — the most congested lane (excluding the currently active one) is picked next; ties are broken by whichever lane has been waiting the longest.

**Dynamic congestion rule** — implemented exactly as defined in the subject:

```
capacity = floor(lane_length / (vehicle_length + safety_gap))
```

- `lane_length`: distance from the spawn point to the stop line
- `vehicle_length`: `CAR_WIDTH`
- `safety_gap`: `SAFETY_GAP`

When a lane's fill ratio (`vehicle_count / capacity`) exceeds 40%, the green time granted to that lane is extended (2s instead of 1s), keeping congestion below the lane's maximum capacity.

### 3. Vehicles

| Color | Route |
|-------|-------|
| 🔴 Red | Turns **right** |
| 🟡 Yellow | Turns **left** |
| 🔵 Blue | Goes **straight** |

Rules respected:

- Color is assigned randomly at creation and **permanently determines** the route — a vehicle cannot change its selected route once created.
- Each vehicle has a **fixed velocity** (`VEHICLE_SPEED`).
- A **safety distance** (`SAFETY_GAP` + vehicle width) is maintained from the vehicle ahead in the same lane; if it stops, the following vehicle stops too before getting too close.
- A vehicle **stops on red** and **proceeds on green**, unless it has already passed the clearing line (in which case it finishes crossing so it doesn't block the intersection).
- No vehicle has special privileges (no emergency vehicles).

## Project Structure

```
src/
├── main.rs      — Entry point, window setup, game loop
├── consts.rs    — Constants (window size, speeds, gaps, etc.)
├── car.rs       — Car, Direction, CarManager (spawning, movement, collision avoidance)
├── light.rs     — TrafficController (light phases, congestion-based timing)
└── road.rs      — Road drawing (surface, lane markings, stop lines)
```

## Key Constants

| Constant | Value | Description |
|----------|-------|-------------|
| Window | 900×900 px | Simulation area |
| Lane width | 60 px | `LANE_WIDTH` |
| Vehicle size | 30×30 px | `CAR_WIDTH` |
| Vehicle speed | 96 px/s | Crosses the window in 10s |
| Safety gap | 25 px | `SAFETY_GAP` |
| Spawn cooldown (same direction) | 0.4 s | `SPAWN_COOLDOWN` |
| All-red clearing phase | 1.5 s | hardcoded in `TrafficController` |

## Building & Running

```bash
# Build
cargo build

# Run
cargo run

# Release build (smoother)
cargo run --release
```

Requires Rust (edition 2024) and the `macroquad` crate.

## Authors

- [bnomenja](https://github.com/bnomenja)
- [yamazzal](https://github.com/YounsseAmazzal)
- [taoussaminee](https://github.com/taoussaminee)