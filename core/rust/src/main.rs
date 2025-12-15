use hermes_core::policy::PolicyEngine;
use hermes_core::mediation::Syscall;
use log::{info, warn, error};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the Lua policy script
    #[arg(short, long, default_value = "scripting/lua/policy.lua")]
    policy: PathBuf,

    /// Target binary (for simulation transparency, just displayed)
    #[arg(default_value = "simulation_target")]
    target: String,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Args::parse();
    
    println!("==================================================");
    println!("   HERMES Binary Runtime - CLI Control Plane      ");
    println!("==================================================");
    println!("[INFO] Policy Path: {:?}", args.policy);
    println!("[INFO] Target:      {}", args.target);
    println!("[INFO] Mode:        Simulation (Windows Host)");

    // 1. Initialize Policy Engine
    let engine = PolicyEngine::new()?;
    
    // 2. Load Policy
    let policy_content = std::fs::read_to_string(&args.policy)
        .map_err(|e| anyhow::anyhow!("Failed to read policy file {:?}: {}", args.policy, e))?;

    println!("[CORE] Loading policy...");
    engine.load_policy(&policy_content)?;
    println!("[CORE] Policy loaded successfully.");

    // 3. Simulate Execution Stream
    // This represents what the C interceptor would send to Rust
    let events = vec![
        (Syscall::Open, [0, 0, 0, 0, 0, 0], "Opening /etc/passwd"),
        (Syscall::Read, [3, 1024, 0, 0, 0, 0], "Reading file content"),
        (Syscall::Execve, [0, 0, 0, 0, 0, 0], "Attempting execve(/bin/sh)"),
        (Syscall::Connect, [0, 0, 0, 0, 0, 0], "Connecting to 1.2.3.4"),
    ];

    println!("\n[CORE] Beginning Event Stream Processing:");
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

    println!("\n[CORE] Simulation Complete.");
    Ok(())
}
