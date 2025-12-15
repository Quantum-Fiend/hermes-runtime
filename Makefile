# Makefile for HERMES Binary Runtime

# Directories
CORE_DIR = core
RUST_DIR = $(CORE_DIR)/rust
INTERCEPT_DIR = intercept/c
BUILD_DIR = build

# Tools
CC = gcc
CFLAGS = -Wall -Wextra -O2 -I$(CORE_DIR)/include
RUSTC = cargo

# Artifacts
TARGET = $(BUILD_DIR)/hermes_runner
LIB_CORE = $(RUST_DIR)/target/release/libhermes_core.a

.PHONY: all clean core cli

all: core cli

# 1. Build Rust Core (Static Library for Linking)
core:
	@echo "[BUILD] Building Rust Core..."
	cd $(RUST_DIR) && $(RUSTC) build --release

# 2. Build CLI / Simulator
cli:
	@echo "[BUILD] Building Simulation CLI..."
	cd $(RUST_DIR) && $(RUSTC) build --bin hermes

# 3. Build Full Runtime (Linux Only - Requires Seccomp)
runtime: core
	@echo "[BUILD] Building C Interceptor..."
	mkdir -p $(BUILD_DIR)
	# Note: This requires -lseccomp and -lpthread, and linking against Rust lib
	# This is a placeholder command for reference
	# $(CC) $(CFLAGS) $(INTERCEPT_DIR)/ptrace_core.c -L$(RUST_DIR)/target/release -lhermes_core -o $(TARGET)

clean:
	cd $(RUST_DIR) && $(RUSTC) clean
	rm -rf $(BUILD_DIR)

help:
	@echo "HERMES Build System"
	@echo "-------------------"
	@echo "make core    - Build Rust core library"
	@echo "make cli     - Build the simulation CLI (Cross-platform)"
	@echo "make runtime - Build the full Linux interceptor (Requires Linux)"
