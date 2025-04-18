#include <assert.h>
#include <fcntl.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int main(int argc, char *argv[])
{
    assert(argc == 2);
    int n = atoi(argv[1]);
    const char *path = "test.txt";
    // stdin  0th fd
    // stdout 1st fd
    // stderr 2nd fd
    // follow 3rd fd
    int fd = open(path, O_RDONLY);
    // printf("fd: %d\n", fd);

    for (size_t i = 4; i < n; i++) {
        int fd2 = 1000 + i;
        int result = dup2(fd, fd2);
        if (result == -1) {
            fprintf(stderr, "ERROR: can not use %d file descriptors\n", n);
            exit(1);
        }
    }
    printf("Success!\n");

    return 0;
}
