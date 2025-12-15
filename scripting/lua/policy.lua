-- HERMES Default Policy
-- This script is loaded by the Rust core at runtime.

function on_syscall(name, args)
    print("[LUA POLICY] Analyzing syscall: " .. name)
    
    -- Arguments are passed as a table (1-based index)
    -- Example: args[1] is the first syscall argument (rdi on x86_64)
    if args and #args > 0 then
        print("[LUA POLICY] Args: " .. table.concat(args, ", "))
    end

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
