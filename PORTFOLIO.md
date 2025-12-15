# HERMES: Portfolio Summary

**Project Title:** HERMES Binary Mediation Runtime
**Tagline:** A policy-driven execution monitor for intercepting and reshaping binary behavior without source code access.

## 🚀 The Core Problem
In cybersecurity and legacy systems operations, we often need to control how a program behaves (e.g., blocking network access, virtualizing file paths, inspecting encryption keys) *without* having the source code to recompile it. Existing tools like `strace` are passive observers, and full sandboxes are often too heavy or rigid.

## 💡 The Solution
HERMES is a **semantic mediation layer** that sits between the process and the kernel. It uses `ptrace` and `seccomp` to intercept system calls, lifting them into a safe **Rust** runtime where **Lua** policies decide whether to Allow, Block, or Modify the action in real-time.

## 🛠️ Technical Highlights

### 1. Hybrid Architecture (Safety & Speed)
*   **C / Assembly / Seccomp**: Handles the high-speed, unsafe context switching at the OS boundary.
*   **Rust Core**: Provides memory safety for state tracking (file descriptors, memory maps) and policy evaluation. This prevents the "monitor" from introducing its own vulnerabilities.
*   **Lua Scripting**: Embeds a hot-reloadable logic engine, allowing operators to change rules without restarting the monitor.

### 2. Zero-Copy Context Modeling
The runtime maps raw CPU registers (e.g., `RAX`, `RDI`, `RSI` on x86_64) into semantic Rust structs (`Syscall::Open`, `Syscall::Connect`) with zero overhead, enabling high-level reasoning about low-level execution streams.

### 3. "Opaque" Binary Control
Demonstrates the ability to take a closed-source binary (simulated in the `demo/` folder) and:
*   **Virtualize** its file access (redirecting `/etc/hosts` to a sandbox file).
*   **Firewall** its network calls based on dynamic rules.
*   **Inject Faults** to test resilience.

## 🏗️ Architecture Stack
*   **Systems Programming**: Rust, C, x86_64 Assembly
*   **OS Internals**: Ptrace, Seccomp-BPF, ELF Analysis
*   **Scripting Integration**: Lua C API (via `mlua`)
*   **Tooling**: Makefile, Python Trace Analysis

## 🎯 Why This Matters
This project demonstrates deep competency in **Systems Programming** and **kernel-user space interaction**. It moves beyond simple CRUD apps to tackle fundamental execution control, showing understanding of ABIs, memory safety, and runtime instrumentation.
