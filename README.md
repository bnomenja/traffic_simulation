# Traffic Control Simulation

A real-time 4-way traffic intersection simulation built with [Rust](https://www.rust-lang.org/) and [Macroquad](https://macroquad.rs/).

![Screenshot](screenshot.png)

## Overview

Vehicles approach a four-way intersection from all directions. A congestion-aware traffic light controller cycles through directions, giving priority to the busiest lanes. Each car is assigned a random color that determines its behavior at the intersection:

| Color | Behavior |
|-------|----------|
| Red | Turns right |
| Yellow | Turns left |
| Blue | Goes straight |

## Controls

| Key | Action |
|-----|--------|
| `↑` | Spawn car from North |
| `↓` | Spawn car from South |
| `→` | Spawn car from East |
| `←` | Spawn car from West |
| `R` | Spawn car from a random direction |
| `C` / `Backspace` | Clear all cars |
| `Esc` | Quit |

## How It Works

### Traffic Light System

Only one direction is green at a time. The controller uses three phases:

- **Green phase** — cars in the active direction proceed through the intersection. Duration scales with lane congestion (0.5s for empty, 1–2s for busy lanes).
- **All-red clearing phase** — 1.5 seconds where all lights are red, allowing cars already in the intersection to clear before the next direction gets green.
- **Next lane selection** — the most congested lane (excluding the currently active one) is chosen next.

### Car Behavior

- Cars stop at their lane's stop line when the light is red.
- Cars past the *clear line* continue through the intersection regardless of the light (committed to crossing).
- In designated turn zones, cars snap to the center of the crossing lane and change direction.
- Same-direction collision avoidance: a car stops if another car of the same direction is ahead within the safety gap.

### Intersection Layout

```
       │  ↑  │
       │  N  │
───────┼─────┼───────
  W ←  │  ·  │  → E
───────┼─────┼───────
       │  S  │
       │  ↓  │
```

## Project Structure

```
src/
├── main.rs      — Entry point, window setup, game loop
├── consts.rs    — Constants (window size, speeds, gaps, etc.)
├── car.rs       — Car struct, Direction enum, CarManager (spawning + collision)
├── light.rs     — TrafficController (light phases, congestion-based timing)
└── road.rs      — Road drawing (surface, lane markings, dashed lines)
```

## Key Constants

| Constant | Value | Description |
|----------|-------|-------------|
| Window | 900×900 px | Simulation area |
| Lane width | 60 px | Width of each road lane |
| Car size | 30×30 px | Vehicle dimensions |
| Vehicle speed | 96 px/s | Based on 10s to cross the full window |
| Safety gap | 25 px | Minimum edge-to-edge distance between cars |
| Spawn cooldown | 0.4 s | Minimum time between spawns from the same direction |

## Building & Running

```bash
# Build
cargo build

# Run
cargo run

# Release build (faster)
cargo run --release
```

Requires Rust 1.80+ (edition 2024).

## Credits

Built as a team project exploring real-time simulation, traffic flow, and concurrent systems in Rust.
