# HERMES: Demo Video Script

**Goal:** Record a 2-minute video demonstrating HERMES for your portfolio.
**Prerequisites:** VS Code, Terminal, OBS (or Loom).

## Scene 1: Introduction (0:00 - 0:30)
*   **Visual:** Show the `README.md` on GitHub or VS Code.
*   **Voiceover:**
    > "Hi, I'm [Your Name], and this is HERMES. It's a binary mediation runtime I built to control program execution without needing source code. It sits between the kernel and the application, intercepting system calls in real-time."
    > "Unlike a debugger, which is passive, HERMES uses a Policy Engine written in Rust and Lua to actively block or modify behavior."

## Scene 2: The Architecture (0:30 - 0:50)
*   **Visual:** Open `docs/architecture.md` (or the Mermaid diagram).
*   **Voiceover:**
    > "The system has three layers. The **Interceptor** in C uses `ptrace` and `seccomp` to catch syscalls. It hands control to the **Rust Core**, which ensures memory safety. Finally, users define logic in **Lua**, allowing for hot-reloadable policies."

## Scene 3: The Code (0:50 - 1:10)
*   **Visual:** Split screen. Left: `core/rust/src/lib.rs` (Show `Syscall` enum). Right: `scripting/lua/examples/firewall.lua`.
*   **Voiceover:**
    > "Here you can see the Rust core mapping raw x86 registers into high-level Enums. On the right is a Lua policy. It's incredibly simple: if the program tries to `Connect` to a restricted port, we return `BLOCK`."

## Scene 4: The Live Demo (1:10 - 1:45)
*   **Visual:** Open Terminal.
*   **Action 1 (Fail Case):**
    *   Run simulation with a "BLOCK" policy.
    *   `./run_simulation.bat` (or Linux equivalent).
    *   Show output: `[EVENT] Connect -> Decision: Block`.
*   **Action 2 (Success Case):**
    *   Edit the Lua file to "ALLOW".
    *   Run again.
    *   Show output: `[EVENT] Connect -> Decision: Allow`.
*   **Voiceover:**
    > "I'm running the simulation mode here. You can see it intercepts the connection attempt. With one line of Lua, I can firewall a binary that I didn't even compile."

## Scene 5: Conclusion (1:45 - 2:00)
*   **Visual:** Back to `PORTFOLIO.md`.
*   **Voiceover:**
    > "HERMES demonstrates how to combine low-level systems programming with high-level scripting to build safe, performant tools. Thanks for watching."
