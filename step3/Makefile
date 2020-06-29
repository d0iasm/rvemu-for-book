csr.bin: csr.s
	riscv64-unknown-elf-gcc -Wl,-Ttext=0x0 -nostdlib -o csr csr.s
	riscv64-unknown-elf-objcopy -O binary csr csr.bin

clean:
	rm -f csr
	rm -f csr.bin
