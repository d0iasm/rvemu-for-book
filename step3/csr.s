main:
  addi t0, zero, 1
  addi t1, zero, 2
  addi t2, zero, 3
  csrrw zero, mstatus, t0
  csrrs zero, mtvec, t1
  csrrw zero, mepc, t2
  csrrc t2, mepc, zero
  csrrwi zero, sstatus, 4
  csrrsi zero, stvec, 5
  csrrwi zero, sepc, 6
  csrrci zero, sepc, 0
