#ifndef HERMES_API_H
#define HERMES_API_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// Rust Core Entry Points
void hermes_init();
int hermes_on_syscall_enter(int pid, long syscall_nr, uint64_t args[6]);
int hermes_on_syscall_exit(int pid, long syscall_nr, long ret_val);

// C Interceptor Entry Points
int hermes_attach(int pid);
int hermes_launch_process(const char* path, char* const argv[]);

#ifdef __cplusplus
}
#endif

#endif // HERMES_API_H
