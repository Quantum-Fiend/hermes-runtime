# HERMES Architecture Design

## Overview

HERMES utilizes a layered architecture to separate low-level execution control from high-level policy logic. The primary goal is to ensure safety and stability while enabling powerful runtime modification.

## Layers

### 1. The Interceptor (Low-Level)
**Languages:** C, Assembly (x86_64)

The Interceptor is the "hands" of HERMES. It touches the hostile binary directly.
It uses **Ptrace** for process control and **Seccomp-BPF** for low-overhead syscall filtering.

*   **Responsibility:**
    *   Attach to target process.
    *   Set up seccomp filters to trap only interesting syscalls.
    *   Handle `SIGTRAP` and `PTRACE_EVENT` signals.
    *   Read/Write process memory (PTRACE_PEEKDATA, PTRACE_POKEDATA).
    *   Read/Write registers (PTRACE_GETREGS, PTRACE_SETREGS).

### 2. The Core (Mediation & Safety)
**Language:** Rust

The Core is the "brain" of the runtime. It models the state of the execution and guarantees safety.
It receives raw events from the Interceptor and converts them into semantic `ExecutionEvent` objects.

*   **Responsibility:**
    *   **State Modeling:** Tracks open file descriptors, memory maps, and child processes.
    *   **Abstraction:** Converts `rax=2` (on x86_64) to `Syscall::Open`.
    *   **Policy Dispatch:** Consults the Policy Engine to decide what to do.
    *   **Action Enforcement:** Instructs the Interceptor to Resume, Block, or Modify.

### 3. The Scripting Engine (Dynamic Policy)
**Language:** Lua (embedded in Rust via `rlua` or similar)

The Scripting Engine allows users to define logic without recompiling the core.

*   **Responsibility:**
    *   Load `.lua` policy files.
    *   Expose a restricted API to script writers (e.g., `hermes.deny()`, `hermes.modify_arg()`).
    *   Allow hot-reloading of logic.

## Data Flow (Syscall Interception)

1.  **Target** executes `open("/etc/passwd", O_RDONLY)`.
2.  **Seccomp** filter triggers, suspending the process.
3.  **OS** sends signal to HERMES (tracer).
4.  **Interceptor (C)** catches signal, reads registers.
5.  **Core (Rust)** maps registers to `Syscall::Open` struct.
6.  **Core** queries **Lua Engine**: "Can process X open /etc/passwd?"
7.  **Lua Policy** returns `Action::Deny("Forbidden file")`.
8.  **Core** translates `Deny` to an error code injection (e.g., `-EACCES`).
9.  **Interceptor** writes `-EACCES` to `rax`, skips the actual syscall instruction.
10. **Target** receives "Permission Denied".

## Memory Safety & Isolation

*   **Rust** ensures that internal state (maps, file tracking) is not corrupted.
*   The **Target** runs in its own address space; HERMES runs in a separate process (tracer).
*   Communication happens strictly via kernel APIs (ptrace), preventing the target from crashing HERMES.
