# AGENTS

This document describes how work is done in this repository.
It is intended for contributors and for future maintainers.

This file does not distinguish between human and LLM contributors.

## Mental Model

This project is developed using a brick-and-invariant model.

- **Bricks** are small, time-boxed execution steps.
- **Invariants** are stable guarantees about the system.
- **Commits** publish invariants; they do not record development steps.

## Bricks vs Commits

- Bricks may produce messy, exploratory commits.
- Bricks are not expected to map 1:1 to commits.
- Only invariant-based commits are merged to `master`.

## Invariants

An invariant is a statement about the system that:
- was false before a change,
- is true after the change,
- is expected to remain true unless deliberately changed.

Example invariants include:
- Snapshots are persisted and retrievable by stable URL.
- User input is rendered safely (no XSS).
- The service runs and persists data in a container.

## Branching Model

- `master` contains curated, invariant-based commits only.
- Feature work happens on topic branches.
- Feature branches may contain messy or WIP commits.
- History is rewritten on feature branches once invariants stabilize.
- A branch may establish one or more invariants, which are
  published as separate commits when appropriate.
- Squash merges are not used.

## Merge Commits

- Merge commits summarize **intent**, not implementation.
- They explain what was merged and why it matters.

## Shipping Philosophy

- `master` is treated as a published contract.
- Clarity of history is preferred over chronological accuracy.
