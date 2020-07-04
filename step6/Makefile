helloworld.bin: helloworld.c
	riscv64-unknown-elf-gcc -S helloworld.c
	riscv64-unknown-elf-gcc -Wl,-Ttext=0x0 -nostdlib -o helloworld helloworld.s
	riscv64-unknown-elf-objcopy -O binary helloworld helloworld.bin

clean:
	rm -f helloworld.s
	rm -f helloworld
	rm -f helloworld.bin
	rm -f echoback.s
	rm -f echoback
	rm -f echoback.bin
