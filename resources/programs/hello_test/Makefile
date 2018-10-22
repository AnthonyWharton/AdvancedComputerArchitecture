# Copyright Anthony Wharton 2018

# Program name for riscv64-linux-gnu-gcc (GCC) 8.2.0
CC=riscv64-linux-gnu-gcc
# Program used for RISC V object dumps
OBJDUMP=riscv64-linux-gnu-objdump
# Standard C Flags, including those to force the ABI/ISA specification
CFLAGS=-mabi=ilp32 -march=rv32im -nostdlib -s -static -O0
# Flags for the linker to not include the build ID binary
LFLAGS=-Wl,--build-id=none
# Dynamically works out the enabled optimisations for our C environment and
# disables them.
# Removes some edge cases that break building (double negatives and c++ flags)
DISABLE_C_OPTIMISATIONS=$(shell                                           \
			  $(CC) $(CFLAGS) $(LFLAGS) -Q --help=optimizers |\
			  grep enabled                                   |\
			  sed -r 's/ *-f([a-zA-Z-]*) .*/-fno-\1/'        |\
			  grep -v 'no-no\|rtti'                          |\
			  tr '\n' ' '                                    )

# Builds the program
build: main.c
	$(CC) $(CFLAGS) $(LFLAGS) main.c

# Disassembles the output file
disassemble: *.out
	${OBJDUMP} -D -z $?

# Cleans the output files
clean: *.out
	rm -rf $?

