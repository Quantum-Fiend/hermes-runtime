# Contributing to HERMES

## Development Philosophy
HERMES operates at the intersection of **safety** (Rust) and **control** (C/asm). Every contribution must respect the isolation boundaries between these layers.

## Safety Guidelines
1.  **Unsafe Code**: Allowed ONLY in `intercept/` and strictly wrapped FFI boundaries.
2.  **No Allocations in Critical Path**: The interception loop must be low-latency.
3.  **Panic Free**: The Rust core must never panic. Use `Result` for all fallible operations.

## Environment Setup
*   **Operating System**: Linux (Kernel 5.4+) required for Seccomp/Ptrace.
*   **Rust**: Stable toolchain.
*   **Dependencies**: `libseccomp-dev`, `lua5.4-dev`.

## Pull Request Process
1.  Ensure `cargo test` passes (covers Policy Engine logic).
2.  If modifying syscall handling, run the `demo/opaque_client` integration test.
3.  Document new policy hooks in `docs/syscall_model.md` (if applicable).
