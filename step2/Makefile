fib.text: fib.c
	riscv64-unknown-elf-gcc -nostdlib -S fib.c
	riscv64-unknown-elf-gcc -Wl,-Ttext=0x0 -nostdlib -o fib fib.s
	riscv64-unknown-elf-objcopy -O binary fib fib.text

clean:
	rm -f fib
	rm -f fib.text
