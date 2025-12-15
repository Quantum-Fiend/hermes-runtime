# How to Run the Demos

This directory contains the target binary and simulation scenarios.

## 1. Building the Demo Target (Linux)

You must compile the opaque client to test real interception.

```bash
gcc -o client demo/opaque_client/client.c
```

## 2. Using the Different Policies

HERMES allows hot-swapping policies. To use a specific example:

**Network Firewall:**
```bash
./hermes run --policy scripting/lua/examples/firewall.lua ./client
```

**File System Virtualization:**
```bash
./hermes run --policy scripting/lua/examples/file_redirect.lua ./client
```

## 3. Simulator (Windows/Mac)

If you are not on Linux, use the Rust simulator to see how the policy engine *would* react to syscall events.

```bash
cd core/rust
cargo run --bin hermes
```
