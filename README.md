# beta-test-alex

Beta test for SuperInstance ternary agent crates — NPC behavior evolution

## Overview

Beta-test-alex: Integration scenario for SuperInstance ternary agent crates.

Simulates evolving NPC behavior strategies for a Roblox-style game.

## Architecture

This crate sits within the **five-layer Oxide Stack**:

| Layer | Crate | Role |
|-------|-------|------|
| 1 | open-parallel | Async runtime (tokio fork) |
| 2 | pincher | "Vector DB as runtime, LLM as compiler" |
| 3 | flux-core | Bytecode VM + A2A agent protocol |
| 4 | cuda-oxide | Flux→MIR→Pliron→NVVM→PTX compiler |
| 5 | cudaclaw | Persistent GPU kernels, warp consensus, SmartCRDT |

The key insight: **ternary values {-1, 0, +1} map directly to GPU compute**. They pack 16× denser than FP32, enable XNOR+popcount matmul, and conservation laws become compile-time checks.

## Stats

| Metric | Value |
|--------|-------|
| Tests | 0
0 |
| Lines of Code | 254 |
| Public API Surface | 21 items |
| License | Apache-2.0 |

## Installation

```toml
[dependencies]
beta-test-alex = "0.1.0"
```

## Usage

```rust
use beta_test_alex::*;
// See src/lib.rs tests for complete working examples
```

### Key Types

```
- pub enum Trit {
    pub fn from_i8(v: i8) -> Self {
    pub fn random(rng: &mut impl Rng) -> Self {
- pub struct TernaryGenome {
    pub fn random(len: usize, rng: &mut impl Rng) -> Self {
    pub fn fitness(&self) -> f64 {
    pub fn mutate(&self, rate: f64, rng: &mut impl Rng) -> Self {
    pub fn len(&self) -> usize {
- pub enum Species {
- pub fn classify_species(genome: &TernaryGenome) -> Species {
```

## Design Philosophy

This crate uses **ternary algebra** (Z₃) where every value is {-1, 0, +1}:

- **+1** → positive signal (healthy, allocated, converged, ready)
- **0** → neutral (pending, balanced, monitoring, degraded)
- **-1** → negative signal (failed, free, diverged, overloaded)

This isn't arbitrary — ternary is the natural encoding for:
1. **BitNet b1.58** (Microsoft) — ternary neural networks at 60% less power
2. **GPU warp voting** — hardware ballot instructions return ternary consensus
3. **Conservation laws** — {-1, 0, +1} preserves quantity (what goes in must come out)

## Testing

```bash
git clone https://github.com/SuperInstance/beta-test-alex.git
cd beta-test-alex
cargo test
```

## License

Apache-2.0
