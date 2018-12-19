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

CHECK_CC:=$(shell $(CC) --version 2> /dev/null)
CHECK_OBJDUMP:=$(shell $(OBJDUMP) --version 2> /dev/null)

# Builds the program
build: main.c
ifdef CHECK_CC
	$(CC) $(CFLAGS) $(LFLAGS) main.c
else
	echo -e "\e[31mIt looks like this machine doesn't have riscv64-linux-gnu-gcc"
	echo -e "Either install this, or use the pre-compiled binary.\e[0m"
endif

# Disassembles the output file
disassemble: *.out
ifdef CHECK_OBJDUMP
	${OBJDUMP} -D -M no-aliases -z $?
else
	echo -e "\e[31mIt looks like this machine doesn't have riscv64-linux-gnu-objdump"
	echo -e "Unable to disassemble the binary.\e[0m"
endif

# Cleans the output files
clean: *.out
ifdef CHECK_CC
	rm -rf $?
else
	echo -e "\e[31mIt looks like this machine doesn't have riscv64-linux-gnu-gcc"
	echo -e "Aborting cleanup, you won't be able to recompile this binary!\e[0m"
endif
