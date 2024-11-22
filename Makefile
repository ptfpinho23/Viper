SHELL := /bin/bash

RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
$(eval $(RUN_ARGS):;@:)

.PHONY: echo
echo:
	echo '-> ' $(RUN_ARGS)


.PHONY: build
build:
	cargo build

.PHONY: generate
generate:
	cargo run


.PHONY: test
test:
	# run compiler
	cargo run

	# assemble stage
	nasm -f elf64 output.asm -o output.o
	
	# linking stage
	ld -m elf_x86_64 output.o -o output

	# attempt to exec
	./output

.PHONY: benchmark
benchmark:
	@echo "benchmarking...."
	@start_time=$$(date +%s%N); \
	python3 ./benchmarks/test.py > /dev/null 2>&1;\
	end_time=$$(date +%s%N); \
	elapsed=$$((end_time - start_time)); \
	echo "python exec  time: $$((elapsed / 1000000)) ms"

	@start_time=$$(date +%s%N); \
	cargo run > /dev/null 2>&1; \
	nasm -f elf64 output.asm -o output.o; \
	ld -m elf_x86_64 output.o -o output; \
	start_time=$$(date +%s%N); \
	./output > /dev/null 2>&1; \
	end_time=$$(date +%s%N); \
	elapsed=$$((end_time - start_time)); \
	echo "Viper exec time: $$((elapsed / 1000000)) ms"

	@gcc -O3 ./benchmarks/test.c -o example_c
	@start_time=$$(date +%s%N); \
	./example_c > /dev/null 2>&1; \
	end_time=$$(date +%s%N); \
	elapsed=$$((end_time - start_time)); \
	echo "C execution time: $$((elapsed / 1000000)) ms"

.PHONY: clean
clean:
	rm -rf ./output ./output.asm ./output.o
