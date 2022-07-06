
# the main program
Main.exe: src/Main.rs libFib.rlib
	rustc $< --extern rary=libFib.rlib

# the library
libFib.rlib: lib/Fib.rs
	rustc --crate-type=lib $<
