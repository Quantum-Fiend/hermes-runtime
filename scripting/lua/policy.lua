-- HERMES Default Policy
-- This script is loaded by the Rust core at runtime.

function on_syscall(name)
    print("[LUA POLICY] Analyzing syscall: " .. name)

    -- Example: Prevent any process from reading /etc/shadow
    -- Note: Real implementation would pass arguments to Lua
    if name == "Open" then
        -- We return "ALLOW", "BLOCK", "MODIFY"
        return "ALLOW"
    end

    if name == "Execve" then
        print("[LUA POLICY] WARNING: Process attempting execution!")
        return "ALLOW"
    end

    return "ALLOW"
end
