CFLAGS=-Wall -Wextra -Wswitch-enum -std=c11 -pedantic
LIBS=

all:
	$(CC) $(CFLAGS) main.c -o curn $(LIBS)

clean:
	@rm -rf curn

.PHONY: all clean