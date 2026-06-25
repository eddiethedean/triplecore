# Contributing to TripleCore

Thank you for your interest in contributing!

## Development setup

1. Install Rust (stable) via [rustup](https://rustup.rs/)
2. Clone the repository
3. Run `cargo test` to verify the workspace builds

Optional: install [just](https://github.com/casey/just) for convenience commands.

## Workflow

1. Create a feature branch from `main`
2. Make focused changes — one crate or concern at a time
3. Run `just fmt`, `just lint`, and `just test` before opening a PR
4. Update docs and CHANGELOG when behavior changes

## Code style

- Follow existing naming and module structure
- Prefer minimal, strongly typed public APIs
- Add tests for real behavior, not trivial assertions
- Use `#![deny(missing_docs)]` on public umbrella APIs
- **Prefer established crates** for RDF syntax, IRI handling, SPARQL parsing, and schema validation — wrap them in adapters rather than reimplementing; see [docs/dependencies.md](docs/dependencies.md)

## Architecture decisions

Significant design choices should be recorded in `docs/decisions/` as ADRs.

## License

By contributing, you agree that your contributions will be licensed under the same dual MIT/Apache-2.0 license as the project.
