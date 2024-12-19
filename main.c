#define _GNU_SOURCE
#include <sched.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mount.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

const char *child_hostname = "container";

static void cmd(int argc, char **argv)
{
    for (int i = 0; i < argc; i++) {
        printf("%s ", argv[i]);
    }
}

static void run_child(int argc, char **argv)
{
    printf("Child Running ");
    cmd(argc, argv);
    printf("as %d\n", getpid());

    int flags = CLONE_NEWUTS | CLONE_NEWNS;

    if (unshare(flags) < 0) {
        fprintf(stderr, "Fail to unshare in child\n");
        exit(1);
    }

    if (mount(NULL, "/", NULL, MS_SLAVE | MS_REC, NULL) < 0) {
        fprintf(stderr, "Fail to mount /\n");
        exit(1);
    }

    if (chroot("../ubuntu-fs") < 0) {
        fprintf(stderr, "Fail to chroot\n");
        exit(1);
    }

    if (chdir("/") < 0) {
        fprintf(stderr, "Fail to child to /\n");
        exit(1);
    }

    if (mount("proc", "proc", "proc", 0, NULL) < 0) {
        fprintf(stderr, "Fail to mount /proc\n");
        exit(1);
    }

    if (sethostname(child_hostname, strlen(child_hostname)) < 0) {
        fprintf(stderr, "Fail to change hostname\n");
        exit(1);
    }

    if (execvp(argv[0], argv)) {
        fprintf(stderr, "Fail to exec\n");
        exit(1);
    }
}

static void run(int argc, char **argv)
{
    printf("Parent Running ");
    cmd(argc, argv);
    printf("as %d\n", getpid());

    if (unshare(CLONE_NEWPID) < 0) {
        fprintf(stderr, "Fail to unshare pid namespace\n");
        exit(1);
    }

    pid_t child_pid = fork();

    if (child_pid < 0) {
        fprintf(stderr, "Fail to fork\n");
        exit(1);
    }

    if (child_pid) {
        if (waitpid(child_pid, NULL, 0) < 0) {
            fprintf(stderr, "Fail to wait for child\n");
            exit(1);
        } else {
            printf("Child terminated\n");
        }
    } else {
        run_child(argc, argv);
    }
}

int main(int argc, char **argv)
{
    if (argc < 3) {
        fprintf(stderr, "Too few arguments\n");
        exit(1);
    }

    if (!strcmp(argv[1], "run")) {
        run(argc - 2, &argv[2]);
    }

    return 0;
}
