#include <vmlinux.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>

#define TASK_COMM_LEN 16
#define MAX_LINE_SIZE 80
#define MAX_PATH_LEN  256

SEC("uretprobe/./ubuntu-fs/bin/bash:readline")
int BPF_KRETPROBE(printret, void *ret)
{
    const char ecurn[] = "ecurn";
    char ori[MAX_LINE_SIZE];
    char str[MAX_LINE_SIZE] = {0};
    char comm[TASK_COMM_LEN];
    long n;
    u32 pid;

    if (!ret) {
        return 0;
    }

    bpf_get_current_comm(&comm, sizeof(comm));
    pid = bpf_get_current_pid_tgid() >> 32;
    bpf_probe_read_user_str(ori, sizeof(ori), ret);
    n = bpf_probe_read_user_str(str, sizeof(str), ret);
    if (n < 0) {
        return 0;
    }

    for (int i = 0; i < 5; i++) {
        if (str[i] != ecurn[i]) {
            return 0;
        }
    }
    str[0] = '/';
    str[5] = '/';
    bpf_probe_write_user(ret, str, n + 1);
    bpf_printk("PID %d (%s) read: %s, modified to: %s\n", pid, comm, ori, str);

    return 0;
};

char LICENSE[] SEC("license") = "GPL";
