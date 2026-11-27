#define _GNU_SOURCE
#include <fcntl.h>
#include <link.h>
#include <signal.h>
#include <stdint.h>
#include <stdlib.h>
#include <sys/time.h>
#include <ucontext.h>
#include <unistd.h>
static uintptr_t image_base;
static int output = -1;
static int find_program(struct dl_phdr_info *obj, size_t size, void *unused) {
    (void)size; (void)unused;
    if (obj->dlpi_name == NULL || obj->dlpi_name[0] == '\0') {
        image_base = obj->dlpi_addr;
        return 1;
    }
    return 0;
}
static void record(int signum, siginfo_t *info, void *context) {
    (void)signum; (void)info;
    uintptr_t address = (uintptr_t)((ucontext_t *)context)->uc_mcontext.gregs[REG_RIP] - image_base;
    ssize_t written = write(output, &address, sizeof(address));
    (void)written;
}
__attribute__((constructor))
static void initialize(void) {
    if (!dl_iterate_phdr(find_program, NULL)) _exit(125);
    output = open(getenv("RISTRETTO_IP_SAMPLES"), O_WRONLY|O_CREAT|O_TRUNC|O_APPEND|O_CLOEXEC, 0600);
    if (output < 0) _exit(125);
    struct sigaction action = {.sa_sigaction=record, .sa_flags=SA_RESTART|SA_SIGINFO};
    sigemptyset(&action.sa_mask);
    if (sigaction(SIGPROF, &action, NULL)) _exit(125);
    struct itimerval timer = {.it_interval={0, 7000}, .it_value={0, 7000}};
    if (setitimer(ITIMER_PROF, &timer, NULL)) _exit(125);
}
