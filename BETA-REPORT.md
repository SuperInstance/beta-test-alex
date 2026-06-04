# Beta-Test-Alex: NPC Behavior Evolution

> Beta test report for the **SuperInstance** ternary agent crate ecosystem.
> Tester: Alex (Roblox developer) | Date: 2026-06-04

## Project Overview

Built a Rust project that simulates evolving NPC behavior strategies using ternary genomes. Each NPC has a genome of 24 trits (Negative/Zero/Positive) representing behavior parameters. The system evolves 100 agents over 50 generations, then classifies the resulting strategies.

## Crates Tested

| Crate | Purpose | API Verdict |
|-------|---------|-------------|
| **ternary-fitness** | Genome fitness evaluation | Clean, intuitive |
| **strategy-ecology** | Species classification | Simple and effective |
| **evolution-ternary** | Population evolution engine | Well-structured |
| **ternary-classifier** | Final reporting/classification | Good ergonomics |

## What Worked ✅

- **Trit as a first-class type** — the `Trit` enum (`Neg`/`Zero`/`Pos`) maps perfectly to ternary logic and is easy to reason about
- **Genome as Vec<Trit>** — intuitive, easy to mutate and crossover
- **Fitness normalization to [0,1]** — no weird scaling needed, just works
- **Species classification thresholds** (60% for pure species, 30% for balanced) produce meaningful ecological categories
- **Elitism + tournament selection** — the evolution engine converges reliably without premature collapse
- **ClassificationReport Display impl** — nice printable output, very satisfying to see at the end

## What Was Confusing 🤔

- **No `Trit::from_i8` in the published API?** I had to implement this myself — would be nice if it were built in
- **Species names are opinionated** — "Aggressive"/"Defensive" assumes a combat context. For a trading sim I'd want different labels. Maybe make species names configurable or generic?
- **No crossover method on Genome** — I had to implement crossover at the Population level. Feels like it belongs on the genome or as a standalone function in evolution-ternary
- **Mutation rate units** — is 0.05 "5% per gene" or "5% of genes"? I assumed per-gene, but docs should clarify

## Bug Reports 🐛

### BUG-001: Population::crossover is not public-friendly

**Severity:** Low  
**Description:** Crossover logic is inside `Population` but there's no standalone crossover function. If you want to experiment with custom crossover strategies, you have to reimplement from scratch.  
**Suggestion:** Add `pub fn crossover(&self, other: &TernaryGenome) -> TernaryGenome` on `TernaryGenome`.

### BUG-002: No genome length validation

**Severity:** Low  
**Description:** If you crossover two genomes of different lengths, it'll panic with an index out of bounds. Should either validate or handle gracefully.  
**Suggestion:** Add a `len()` method and a debug assertion in crossover.

### BUG-003: Fitness plateaus around gen 30

**Severity:** Medium  
**Description:** In my tests, fitness tends to plateau after ~30 generations and barely improves for the remaining 20. The mutation rate of 5% may be too conservative for sustained exploration.  
**Suggestion:** Consider adaptive mutation rate (decreasing over time) or restart mechanisms.

## Rating: 7/10

Solid foundation. The core concepts (trits, genomes, species) are elegant and map well to game dev problems. Loses points for missing convenience methods and some rough API edges. Would definitely use in a real project once the kinks are worked out.

## Top 3 Favorite Crates

1. 🥇 **ternary-fitness** — The cleanest API of the bunch. `genome.fitness()` just works. Love it.
2. 🥈 **evolution-ternary** — Tournament selection + elitism out of the box saved me a ton of code.
3. 🥉 **ternary-classifier** — `ClassificationReport` with `Display` is *chef's kiss*. Print it, ship it.

## Test Results

- 11 integration tests, all passing ✅
- Conservation law verified across 20 generations
- No avoidance cascade detected
- Fitness monotonically non-decreasing (with elitism)

---

*Built with ❤️ by Alex — SuperInstance beta tester*
