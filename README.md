# HERMES: Policy-Driven Binary Runtime

[![Build Status](https://img.shields.io/github/actions/workflow/status/username/hermes-binary-runtime/ci.yml?branch=main)](https://github.com/username/hermes-binary-runtime/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**HERMES** is a production-grade binary mediation runtime that intercepts, inspects, and modifies program execution at the kernel boundary—without source code access.

It acts as a programmable firewall for system calls, enabling:
*   **Virtualization**: Redirect file paths and network connections.
*   **Enforcement**: Block dangerous behaviors (e.g., `execve`, sensitive reads).
*   **Instrumentation**: Trace execution flow with zero binary modification.

## 🏗 Architecture

HERMES enforces a strict separation of concerns for stability and safety:

| Layer | Language | Role |
|-------|----------|------|
| **Control Plane** | **Rust** | Memory-safe state tracking, policy dispatch, and error handling. |
| **Interceptor** | **C** | High-performance `ptrace` + `seccomp` event loop. |
| **Policy Engine** | **Lua** | Hot-reloadable logic scripts defining runtime rules. |

## 🚀 Getting Started

### Prerequisites
*   Linux (x86_64) Kernel 5.4+
*   Rust 1.70+
*   `libseccomp-dev`

### Installation
```bash
git clone https://github.com/username/hermes-binary-runtime.git
cd hermes-binary-runtime
make all
```

### Usage
Run a binary under HERMES control with a specific policy:

```bash
# Block specific networks & internal files
./hermes run --policy policies/firewall.lua --target ./your_binary
```

### Simulator (Non-Linux)
Development can be done on Windows/Mac using the Rust-based core simulator:

```bash
./run_simulation.bat
```

## 🛡️ Policy Example (`firewall.lua`)

```lua
function on_syscall(name, args)
    if name == "Connect" then
        print("Blocked connection attempt to restricted subnet.")
        return "BLOCK"
    end
    return "ALLOW"
end
```

## 🤝 Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md) for safety guidelines and architectural standards.
