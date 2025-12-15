@echo off
echo [HERMES] Starting Core Simulator...
echo.

cd core\rust

:: Check if Cargo is available
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERROR] Rust/Cargo is not installed or not in PATH.
    echo Please install Rust from https://rustup.rs/
    echo.
    echo Since you cannot compile the code right now, here is what the output WOULD look like:
    echo.
    echo ==================================================
    echo    HERMES Binary Runtime - Core Simulator (v0.1)  
    echo ==================================================
    echo Running on Windows host.
    echo Starting Policy Engine simulation...
    echo.
    echo [SIM] Loading policy from: scripting/lua/policy.lua
    echo.
    echo [SIM] Beginning Event Stream Processing:
    echo ----------------------------------------
    echo [EVENT] Syscall: Open | Context: Opening /etc/passwd
    echo [LUA POLICY] Analyzing syscall: Open
    echo    -> Decision: Allow
    echo ----------------------------------------
    echo [EVENT] Syscall: Execve | Context: Attempting execve(/bin/sh)
    echo [LUA POLICY] Analyzing syscall: Execve
    echo [LUA POLICY] WARNING: Process attempting execution!
    echo    -> Decision: Allow
    echo ----------------------------------------
    echo.
    echo [SIM] Simulation Complete.
    pause
    exit /b
)

:: Run the real simulator if Rust is present
cargo run --bin hermes
pause
