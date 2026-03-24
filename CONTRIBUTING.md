# Contributing to Saros

## Branch Model

- `main` — protected. No direct commits. All changes via pull request.
- `dev` — integration branch. Feature branches merge here first.
- `feat/<name>` — feature branches.
- `fix/<name>` — bug fix branches.

## Pull Request Rules

- PRs targeting `main` require **1 approving review from `zacxxx`**.
- No self-merge. No bypassing branch protection.
- PR title must follow: `feat:`, `fix:`, `chore:`, `docs:`, `refactor:`.
- Squash merge preferred for feature branches; merge commit for `dev → main`.

## Code Standards

All contributions must follow [`documentation/CODE_RULES.md`](documentation/CODE_RULES.md):

- One function/type per file
- SoA data layout for game components
- No `unwrap()` / `expect()` in production paths — use `?` or explicit error handling
- No tests unless explicitly requested
- Minimal code — no verbose implementations

## Rust

- Run `cargo check` before pushing
- `cargo fmt` and `cargo clippy` must pass

## TypeScript / React / Next.js

- `bun run build` must pass in both `game/` and `devtools/`
- No `any` types without a comment explaining why

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(terrain): add cave carving with 3D Perlin noise
fix(pathfinding): handle unreachable tiles gracefully
chore(deps): bump bevy to 0.13.2
```

## Reporting Issues

Open a GitHub issue with:
- What you expected
- What happened
- Steps to reproduce
- Relevant logs or screenshots
