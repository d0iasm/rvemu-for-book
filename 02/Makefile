fib.bin: fib.c
	riscv64-unknown-elf-gcc -S fib.c
	riscv64-unknown-elf-gcc -Wl,-Ttext=0x0 -nostdlib -o fib fib.s
	riscv64-unknown-elf-objcopy -O binary fib fib.bin

clean:
	rm -f fib
	rm -f fib.bin
