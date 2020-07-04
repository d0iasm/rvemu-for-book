echoback.bin: echoback.c
	riscv64-unknown-elf-gcc -S echoback.c
	riscv64-unknown-elf-gcc -Wl,-Ttext=0x0 -nostdlib -o echoback echoback.s
	riscv64-unknown-elf-objcopy -O binary echoback echoback.bin

clean:
	rm -f echoback.s
	rm -f echoback
	rm -f echoback.bin
