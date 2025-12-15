# HERMES Release v1.0.0

**Release Date:** 2024-12-15
**Status:** Stable / Production Ready

## 📦 Release Highlights

### 🛡️ Hybrid Safety Architecture
HERMES v1.0 establishes the industry-standard "Safety Sandwich" pattern:
1.  **Unsafe Layer**: C/ASM Interceptor (Ptrace/Seccomp)
2.  **Safe Layer**: Rust Mediation Core (Type-safe State)
3.  **Dynamic Layer**: Lua Policy Engine (Sandboxed Logic)

### 🔌 New Capabilities
*   **Hot-Swappable Policies**: Change logic without restarting the runtime.
*   **Granular Syscall Control**: Inspect arguments (IPs, Paths) before deciding.
*   **Cross-Platform Simulator**: Develop policies on Windows/Mac, deploy on Linux.
*   **CLI Configuration**: Full `clap` integration for robust flag handling.

## 🔧 Installation & Usage

**Linux (Production)**
```bash
make all
./hermes --policy scripting/lua/examples/firewall.lua --target ./demo/opaque_client/client
```

**Windows (Development)**
```cmd
run_simulation.bat
```

## 📝 Known Issues
*   `execve` argument modification is currently read-only in the simulator.
*   Multi-threaded target support is experimental.

## 👥 Contributors
*   Tushar (Lead Architect)
*   Antigravity (AI Assistant)
