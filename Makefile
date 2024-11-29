SHELL := /bin/bash

RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
$(eval $(RUN_ARGS):;@:)

# Linker for macos - x86_64-elf-ld; fallback onto gnu linker on linux
UNAME_S := $(shell uname -s)
UNAME_M := $(shell uname -m)
LINKER := ld
ifeq ($(UNAME_S), Darwin)
    LINKER := x86_64-elf-ld
endif

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
	$(LINKER) output.o -o output

	# execute the binary
	@if [ "$(UNAME_M)" = "x86_64" ]; then \
		echo "Running natively on x86_64"; \
		./output; \
	else \
		echo "Running in Docker (emulated)"; \
		docker run --rm -v $(shell pwd):/app -w /app ubuntu:latest ./output; \
	fi

.PHONY: benchmark
benchmark:
	@echo "benchmarking...."
	@start_time=$$(date +%s%N); \
	python3 ./benchmarks/test.py > /dev/null 2>&1; \
	end_time=$$(date +%s%N); \
	elapsed=$$((end_time - start_time)); \
	echo "Python exec time: $$((elapsed / 1000000)) ms"

	@start_time=$$(date +%s%N); \
	cargo run > /dev/null 2>&1; \
	nasm -f elf64 output.asm -o output.o; \
	$(LINKER) output.o -o output; \

	if [ "$(UNAME_M)" = "x86_64" ]; then \
		./output > /dev/null 2>&1; \
	else \
		docker run --rm -v $(shell pwd):/app -w /app ubuntu:latest ./output > /dev/null 2>&1; \
	fi; \

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
