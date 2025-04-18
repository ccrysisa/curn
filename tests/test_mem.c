#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define KB(x) ((x) * 1024)
#define MB(x) ((x) * 1024 * 1024)

int main(int argc, char *argv[])
{
    assert(argc == 2);
    size_t len = strlen(argv[1]);
    char c = argv[1][len - 1];
    argv[1][len - 1] = 0;
    size_t n = atoi(argv[1]);

    if (c == 'M') {
        n = MB(n);
    } else if (c == 'K') {
        n = KB(n);
    } else {
        fprintf(stderr, "ERROR: unknown unit `%c`\n", c);
    }

    char *p = malloc(n);
    if (p == NULL) {
        fprintf(stderr, "ERROR: can not allocate %zu bytes\n", n);
        exit(1);
    }
    for (size_t i = 0; i < n; i++) {
        p[0] = 'c';
    }

    return 0;
}
