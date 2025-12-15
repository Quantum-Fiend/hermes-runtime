# Syscall Model & Policy API

HERMES normalizes architecture-specific syscalls into a platform-independent model.
This allows policies to be written once and (eventually) run on multiple architectures.

## 1. Syscall Abstractions

The Core Runtime maps raw syscall numbers (e.g., `rax` on x86_64) to the `Syscall` Enum.

| Syscall Event | Description | Argument 1 | Argument 2 | Argument 3 |
|---------------|-------------|------------|------------|------------|
| `Open`        | Open a file | `char* path` | `int flags` | `int mode` |
| `Read`        | Read from FD| `int fd`     | `void* buf` | `size_t count` |
| `Write`       | Write to FD | `int fd`     | `void* buf` | `size_t count` |
| `Execve`      | Execute prog| `char* path` | `char** argv`| `char** envp`|
| `Connect`     | Net connect | `int sockfd` | `sockaddr*` | `socklen_t` |

## 2. Lua Policy API

Scripts must implement `on_syscall(name, args)`.

### Arguments
*   **name** (`string`): The mapped syscall name (e.g., "Open").
*   **args** (`table`): A list of raw `u64` arguments (1-based index).

### Return Values
The function must return one of the following strings:

*   `"ALLOW"`: Continue execution normally.
*   `"BLOCK"`: Prevent execution. Returns `-EPERM` (Operation not permitted) to the process.
*   `"MODIFY_ARG:<index>:<value>"`: (Advanced) Overwrite a register value before the kernel sees it.

## 3. Data Flow
1.  **Monitor** (C) catches `SYSCALL_ENTRY`.
2.  **Monitor** reads registers (`rdi`, `rsi`...).
3.  **Rust Core** converts registers to `args` vector.
4.  **Lua Engine** executes `on_syscall`.
5.  **Rust Core** receives decision.
    *   If `BLOCK`: Monitor writes `-1` to syscall number (cancelling it) and writes `-EPERM` to return register (`rax`).
