#define _GNU_SOURCE
#include <sys/ptrace.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <sys/user.h>
#include <sys/syscall.h>
#include <sys/reg.h>
#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include <signal.h>

#include "../../core/include/hermes_api.h"
#include "../seccomp/filter.h"

// Macro to simplify error checking
#define CHECK(expr, msg) do { \
    if ((expr) == -1) { \
        perror(msg); \
        exit(EXIT_FAILURE); \
    } \
} while(0)

int hermes_launch_process(const char* path, char* const argv[]) {
    pid_t pid = fork();

    if (pid < 0) {
        perror("fork");
        return -1;
    }

    if (pid == 0) {
        // Child process (Target)
        
        // 1. Allow tracing
        CHECK(ptrace(PTRACE_TRACEME, 0, NULL, NULL), "PTRACE_TRACEME");

        // 2. Stop self to let parent set options
        raise(SIGSTOP);

        // 3. Apply seccomp filters (if enabled)
        // Note: In a real implementation, we might do this after exec or use PTRACE_O_SUSPEND_SECCOMP
        // enable_seccomp_filtering(); 

        // 4. Exec the target binary
        CHECK(execv(path, argv), "execv");
    } else {
        // Parent process (HERMES Tracer)
        return hermes_attach(pid);
    }
    return 0;
}

int hermes_attach(int pid) {
    int status;
    int in_syscall = 0; // Toggle to track entry vs exit

    // Wait for child to stop (SIGSTOP raised in child)
    waitpid(pid, &status, 0);

    // Set ptrace options
    // PTRACE_O_TRACESYSGOOD: Set bit 7 in status for syscall stops, distinct from other signals
    // PTRACE_O_EXITKILL: If tracer dies, kill tracee
    CHECK(ptrace(PTRACE_SETOPTIONS, pid, 0, 
        PTRACE_O_TRACESYSGOOD | PTRACE_O_EXITKILL), "PTRACE_SETOPTIONS");

    printf("[HERMES] Attached to PID %d. Beginning interception loop.\n", pid);

    while (1) {
        // Continue and stop at next syscall
        CHECK(ptrace(PTRACE_SYSCALL, pid, 0, 0), "PTRACE_SYSCALL");

        // Wait for state change
        waitpid(pid, &status, 0);

        if (WIFEXITED(status)) {
            printf("[HERMES] Target PID %d exited with status %d.\n", pid, WEXITSTATUS(status));
            break;
        }

        if (WIFSIGNALED(status)) {
            printf("[HERMES] Target PID %d killed by signal %d.\n", pid, WTERMSIG(status));
            break;
        }

        // Check if stopped by syscall (bit 7 set)
        if (WIFSTOPPED(status) && (WSTOPSIG(status) & 0x80)) {
            struct user_regs_struct regs;
            CHECK(ptrace(PTRACE_GETREGS, pid, 0, &regs), "PTRACE_GETREGS");

            if (!in_syscall) {
                // SYSCALL ENTRY
                // x86_64: orig_rax holds syscall nr
                uint64_t args[6] = {regs.rdi, regs.rsi, regs.rdx, regs.r10, regs.r8, regs.r9};
                
                // Call Rust Core
                // We forward the decision: 0 = Allow, >0 = Skip/Error
                int action = hermes_on_syscall_enter(pid, regs.orig_rax, args);
                
                if (action != 0) {
                    // Block execution of syscall
                    // On x86_64, setting orig_rax to -1 skips the syscall
                    // But we want to inject an error outcome or return value.
                    // Typically: change syscall nr to -1, set RAX to result.
                    
                    // Note: This is a simplification (regs.orig_rax = -1 might fail on some kernels)
                    // Better technique: change to getpid() (syscall 39) then overwrite return logic on exit.
                }

                in_syscall = 1;
            } else {
                // SYSCALL EXIT
                // RAX holds return value
                long ret_val = regs.rax;

                // Call Rust Core for post-processing
                hermes_on_syscall_exit(pid, regs.orig_rax, ret_val);

                in_syscall = 0;
            }
        } else {
            // Stopped by other signal?
            // printf("[HERMES] Received signal %d\n", WSTOPSIG(status));
        }
    }

    return 0;
}
