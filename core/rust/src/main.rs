use hermes_core::policy::PolicyEngine;
use hermes_core::mediation::Syscall;
use log::{info, warn, error};
use std::env;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    
    println!("==================================================");
    println!("   HERMES Binary Runtime - Core Simulator (v0.1)  ");
    println!("==================================================");
    println!("Running on non-Linux host (Windows detected?).");
    println!("Starting Policy Engine simulation...\n");

    // 1. Initialize Policy Engine
    let engine = PolicyEngine::new()?;
    
    // 2. Load Policy (assuming running from repo root or core/rust)
    // We'll try to find the file in a few common spots for developer convenience
    let policy_paths = [
        "../../scripting/lua/policy.lua",
        "scripting/lua/policy.lua",
        "policy.lua"
    ];

    let mut loaded = false;
    for path in &policy_paths {
        if let Ok(content) = std::fs::read_to_string(path) {
            println!("[SIM] Loading policy from: {}", path);
            engine.load_policy(&content)?;
            loaded = true;
            break;
        }
    }

    if !loaded {
        warn!("[SIM] Could not find policy.lua. Using default empty policy.");
    }

    // 3. Simulate Execution Stream
    // This represents what the C interceptor would send to Rust
    let events = vec![
        (Syscall::Open, [0, 0, 0, 0, 0, 0], "Opening /etc/passwd"),
        (Syscall::Read, [3, 1024, 0, 0, 0, 0], "Reading file content"),
        (Syscall::Execve, [0, 0, 0, 0, 0, 0], "Attempting execve(/bin/sh)"),
        (Syscall::Connect, [0, 0, 0, 0, 0, 0], "Connecting to 1.2.3.4"),
    ];

    println!("\n[SIM] Beginning Event Stream Processing:");
    println!("----------------------------------------");

    for (syscall, args, desc) in events {
        println!("[EVENT] Syscall: {:?} | Context: {}", syscall, desc);
        
        match engine.evaluate(syscall, args) {
            Ok(action) => {
                println!("   -> Decision: {:?}", action);
            },
            Err(e) => {
                error!("   -> Policy Error: {}", e);
            }
        }
        println!("----------------------------------------");
    }

    println!("\n[SIM] Simulation Complete.");
    Ok(())
}
