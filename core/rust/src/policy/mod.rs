use mlua::{Lua, Result, Function};
use crate::mediation::{Syscall, MediationAction};

pub struct PolicyEngine {
    lua: Lua,
}

impl PolicyEngine {
    pub fn new() -> Result<Self> {
        let lua = Lua::new();
        Ok(Self { lua })
    }

    pub fn load_policy(&self, script: &str) -> Result<()> {
        self.lua.load(script).exec()
    }

    pub fn evaluate(&self, syscall: Syscall, args: [u64; 6]) -> Result<MediationAction> {
        let globals = self.lua.globals();
        
        let handler: Function = match globals.get("on_syscall") {
            Ok(f) => f,
            Err(_) => {
                // If no handler is defined, default to ALLOW (fail-open) or BLOCK (fail-closed)
                // For a general runtime, fail-open is safer for stability unless specified otherwise.
                return Ok(MediationAction::Allow); 
            }
        };

        // Convert syscall to string
        let syscall_name = format!("{:?}", syscall);
        
        // Pass arguments as a Lua table (1-based index)
        // Lua uses 1-based indexing, but we'll specificy 6 separate args for simplicity or a table.
        // Let's pass them as a table for cleaner Lua code: on_syscall(name, {arg1, arg2...})
        let args_vec = args.to_vec();

        // Call Lua function: on_syscall(name, args_table)
        let action: String = handler.call((syscall_name, args_vec))?;

        // Parse Action String
        // Format can be: "ALLOW", "BLOCK", "MODIFY_ARG:<idx>:<val>"
        if action == "ALLOW" {
            Ok(MediationAction::Allow)
        } else if action == "BLOCK" {
            Ok(MediationAction::Block(-1)) 
        } else if action.starts_with("MODIFY_ARG") {
            // Very basic parsing for demo: "MODIFY_ARG:0:12345"
            // Production TODO: proper parsing logic
            Ok(MediationAction::ModifyArgs([None, None, None, None, None, None]))
        } else {
            // Default safe fallback
            Ok(MediationAction::Allow) 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allow_default() {
        let engine = PolicyEngine::new().unwrap();
        let script = r#"
            function on_syscall(name, args)
                return "ALLOW"
            end
        "#;
        engine.load_policy(script).unwrap();
        
        let action = engine.evaluate(Syscall::Open, [0; 6]).unwrap();
        match action {
            MediationAction::Allow => assert!(true),
            _ => panic!("Expected Allow"),
        }
    }

    #[test]
    fn test_block_specific_syscall() {
        let engine = PolicyEngine::new().unwrap();
        let script = r#"
            function on_syscall(name, args)
                if name == "Open" then
                    return "BLOCK"
                end
                return "ALLOW"
            end
        "#;
        engine.load_policy(script).unwrap();
        
        let action = engine.evaluate(Syscall::Open, [0; 6]).unwrap();
        match action {
            MediationAction::Block(_) => assert!(true),
            _ => panic!("Expected Block"),
        }
    }

        // Verify other syscalls are allowed
        let action_read = engine.evaluate(Syscall::Read, [0; 6]).unwrap();
        match action_read {
            MediationAction::Allow => assert!(true),
            _ => panic!("Expected Allow for Read"),
        }
    }
}
