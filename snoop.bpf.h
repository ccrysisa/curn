#ifndef __SNOOP_H
#define __SNOOP_H

#define TASK_COMM_LEN 16
#define MAX_LINE_SIZE 80
#define MAX_PATH_LEN  256

struct event {
    int pid;
    int ppid;
    int uid;
    int retval;
    bool is_exit;
    char comm[TASK_COMM_LEN];
};

#endif
