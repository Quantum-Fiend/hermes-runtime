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
        // This is a simplified interface. In a real impl, we'd pass a context object.
        let globals = self.lua.globals();
        
        let handler: Function = match globals.get("on_syscall") {
            Ok(f) => f,
            Err(_) => return Ok(MediationAction::Allow), // No handler defined
        };

        // Convert syscall to string for Lua
        let syscall_name = format!("{:?}", syscall);
        
        // Call Lua function: on_syscall(name, arg1, arg2, ...)
        // For simplicity, just passing name here.
        let action: String = handler.call(syscall_name)?;

        match action.as_str() {
            "BLOCK" => Ok(MediationAction::Block(-1)),
            _ => Ok(MediationAction::Allow),
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
            function on_syscall(name)
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
            function on_syscall(name)
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

        // Verify other syscalls are allowed
        let action_read = engine.evaluate(Syscall::Read, [0; 6]).unwrap();
        match action_read {
            MediationAction::Allow => assert!(true),
            _ => panic!("Expected Allow for Read"),
        }
    }
}
