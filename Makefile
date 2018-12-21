# Copyright 2018 Anthony Wharton
# Simple makefile with a few bindings to get this project running on a fresh
# machine.
ifndef VERBOSE
.SILENT:
endif

# Do not run this makefile in parallel - will not apply to recursive makefile
# calls, so running with -jX may still help.
.NOTPARALLEL:

default:
	echo "This Makefile will (purposefully) not run anything when called with no"
	echo "arguments. Please run one of the following:"
	echo
	echo "PROJECT MANAGEMENT:"
	echo "    make build        - Builds the project."
	echo "    make run          - Runs the simulator."
	echo "    make doc          - Builds and opens the documentation."
	echo "    make clean        - Cleans the project directory."
	echo
	echo "RUST INSTALLATION:"
	echo "    make rust-install - Installs rust to userspace using the packaged rustup"
	echo "                        installer script."
	echo "    make rust-remove  - Unininstalls everything installed by make rust-install."
	echo "    make env          - Sets the environment variable to include rustup binaries"
	echo "                        in your PATH variable."
	echo "Rust installation is powered by rustup, packaged into the project."
	echo "  - https://rustup.rs/"
	echo "  - https://github.com/rust-lang-nursery/rustup.rs"

build:
	$(MAKE) --no-print-directory -C ./resources/programs/ all:build
	echo "================================ simulator"
	cargo build --release

run: build
	./target/release/daybreak --alu=4 --blu=1 --mcu=4 --rsv=32 --rob=64 -n=4 -i=6 -br \
		./resources/programs/fib_non_recursive/a.out

doc:
	cargo doc --document-private-items --open

clean:
	$(MAKE) --no-print-directory -C ./resources/programs all:clean
	echo "================================ simulator"
	cargo clean

rust-install:
	./rustup.sh -y

rust-remove: env
	rustup self uninstall -y

env:
	echo -e "\e[31mSorry this cannot be done automatically, PLEASE RUN:"
	echo -e "\e[1;37msource ~/.cargo/env\e[0m"

