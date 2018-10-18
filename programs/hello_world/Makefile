# Copyright Anthony Wharton 2018

# Program name for riscv64-linux-gnu-gcc (GCC) 8.2.0
CC=riscv64-linux-gnu-gcc
# Program used for RISC V object dumps
OBJDUMP=riscv64-linux-gnu-objdump
# Standard C Flags, including those to force the ABI/ISA specification
CFLAGS=-mabi=ilp32 -march=rv32im -nostdlib -s -static -O0
# Flags for the linker to not include the build ID binary
LINKERFLAGS=-Wl,--build-id=none
# Flags to disable all optimisations in the GCC for RISC V
DISABLE_C_OPTIMISATIONS=-fno-aggressive-loop-optimizations -fno-auto-inc-dec \
			-fno-dce -fno-delete-null-pointer-checks -fno-dse    \
			-fno-early-inlining -fno-fp-int-builtin-inexact      \
			-fno-function-cse -fno-gcse-lm -fno-inline           \
			-fno-inline-atomics -fno-ira-hoist-pressure          \
			-fno-ira-share-save-slots -fno-ira-share-spill-slots \
			-fno-ivopts -fno-jump-tables -fno-lifetime-dse       \
			-fno-math-errno -fno-peephole -fno-plt               \
			-fno-prefetch-loop-arrays -fno-printf-return-value   \
			-fno-reg-struct-return -fno-rename-registers         \
			-fno-sched-critical-path-heuristic                   \
			-fno-sched-dep-count-heuristic                       \
			-fno-sched-group-heuristic -fno-sched-interblock     \
			-fno-sched-last-insn-heuristic                       \
			-fno-sched-rank-heuristic -fno-sched-spec            \
			-fno-sched-spec-insn-heuristic                       \
			-fno-sched-stalled-insns-dep -fno-schedule-fusion    \
			-fno-short-enums -fno-shrink-wrap-separate           \
			-fno-signed-zeros -fno-split-ivs-in-unroller         \
			-fno-ssa-backprop -fno-stdarg-opt                    \
			-fno-strict-volatile-bitfields -fno-trapping-math    \
			-fno-tree-cselim -fno-tree-forwprop                  \
			-fno-tree-loop-if-convert -fno-tree-loop-im          \
			-fno-tree-loop-ivcanon -fno-tree-loop-optimize       \
			-fno-tree-phiprop -fno-tree-reassoc                  \
			-fno-tree-scev-cprop -fno-var-tracking               \
			-fno-var-tracking-assignments -fno-web

# Builds the program
build: main.c
	$(CC) $(CFLAGS) $(DISABLE_C_OPTIMISATIONS) $(LINKERFLAGS) main.c

# Disassembles the output file
disassemble: *.out
	${OBJDUMP} -D -z $?

# Cleans the output files
clean: *.out
	rm -rf $?

