# Changelog

All notable changes to this project will be documented in this file.

## [1.0.0] - 2024-12-15
### Added
- **Core Runtime**: Rust-based safety layer for state tracking.
- **Interceptor**: implementation using `ptrace` and `seccomp` for Linux.
- **Policy Engine**: Embedded Lua 5.4 scripting environment.
- **CLI**: `clap` based argument parsing for `--policy` and `--target`.
- **Simulation Mode**: Windows/Mac compatible logic tester.
- **Documentation**: Architecture docs, Syscall model, and Portfolio summary.
- **CI/CD**: GitHub Actions pipeline for automated testing.

### Security
- Implemented strictly typed `Syscall` enum to prevent invalid state transitions.
- Enforced read-only execution of Lua scripts to prevent side-effects in the monitor.

## [0.1.0] - 2024-12-14
### Initialized
- Project scaffolding and architectural planning.
