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
    echo    HERMES Binary Runtime - CLI Control Plane      
    echo ==================================================
    echo [INFO] Policy Path: "../../scripting/lua/policy.lua"
    echo [INFO] Target:      "simulation_target"
    echo [INFO] Mode:        Simulation (Windows Host)
    echo [CORE] Loading policy...
    echo [CORE] Policy loaded successfully.
    echo.
    echo [CORE] Beginning Event Stream Processing:
    echo ----------------------------------------
    echo [EVENT] Syscall: Open | Context: Opening /etc/passwd
    echo [LUA POLICY] Analyzing syscall: Open
    echo [LUA POLICY] Args: 0, 0, 0, 0, 0, 0
    echo    -> Decision: Allow
    echo ----------------------------------------
    echo [EVENT] Syscall: Execve | Context: Attempting execve(/bin/sh)
    echo [LUA POLICY] Analyzing syscall: Execve
    echo [LUA POLICY] Args: 0, 0, 0, 0, 0, 0
    echo [LUA POLICY] WARNING: Process attempting execution!
    echo    -> Decision: Allow
    echo ----------------------------------------
    echo.
    echo [CORE] Simulation Complete.
    pause
    exit /b
)

:: Run the real simulator if Rust is present
:: Note: The default policy path in src/main.rs assumes a specific CWD.
:: We explicitly pass the path relative to core/rust
cargo run --bin hermes -- --policy ../../scripting/lua/policy.lua --target ../../demo/opaque_client/client
pause
