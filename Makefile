run:
	@cargo run

test:
	@cargo test

compile:
	@gcc -c src/main.c

execute:
	@gcc src/main.c
	./a.out

static-lib: compile
	@ar -rcs lib.a main.o