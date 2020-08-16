# rvemu-for-book
Reference implementation of [the book](https://book.rvemu.app/), *Writing a RISC-V Emulator from Scratch in 10 Steps*. The goal of this code and the book is runnning [xv6](https://github.com/mit-pdos/xv6-riscv) in our emulator.

This is based on original RISC-V emulator [rvemu](https://github.com/d0iasm/rvemu) in Rust.

## How to run xv6
```
$ cd step10 // move to the step10 directory
$ cargo run ./xv6-kernel.bin ./xv6-fs.img
```

![demo.png](https://raw.githubusercontent.com/d0iasm/rvemu-for-book/master/demo.png)

## Step to implement a RISC-V emulator
See https://book.rvemu.app/
- Step 1: Setup and Implement Two Instructions
- Step 2: RV64I Base Integer Instruction Set
- Step 3: Control and Status Registers
- Step 4: Privileged Instruction Set
- Step 5: Exceptions
- Step 6: UART (a universal asynchronous receiver-transmitter)
- Step 7: PLIC (a platform-level interrupt controller) and CLINT (a core-local interrupter)
- Step 8: Interrupts
- Step 9: Virtio
- Step 10: Virtual Memory System
