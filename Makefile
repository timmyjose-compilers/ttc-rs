CC := gcc
CFLAGS := -Wall -std=c99 -flto -O3
EXE_FILE := out.c
EXE := ttc

all:
	$(CC) $(CFLAGS) -o $(EXE) $(EXE_FILE)

.PHONY: clean
clean:
	rm -f $(EXE_FILE) $(EXE)