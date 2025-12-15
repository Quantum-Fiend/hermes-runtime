-- Example: File Redirection (Virtualization)
-- Redirects access from sensitive files to sandboxed versions

function on_syscall(name)
    if name == "Open" then
        -- Simulated check of the path argument
        -- local path = hermes.read_string(args[0])
        
        -- Logic: If trying to read /etc/shadow, redirect to /tmp/fake_shadow
        print("[FS] File access request detected.")
        
        -- return "MODIFY_ARG_0:/tmp/fake_shadow"
        
        return "ALLOW"
    end
    
    return "ALLOW"
end
