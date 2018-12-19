unsigned int fib(unsigned int n) {
	if (n == 0) {
		return 0;
	} else if (n <= 2) {
		return 1;
	} else {
		return fib(n-1) + fib(n-2);
	}
}

void _start()
{
	volatile unsigned int result = fib(9);
	result += 0;
}

/*

00010054 <.text>:
   10054:       ff010113                addi    sp,sp,-16
   10058:       00812423                sw      s0,8(sp)
   1005c:       00912223                sw      s1,4(sp)
   10060:       01212023                sw      s2,0(sp)
   10064:       00112623                sw      ra,12(sp)
   10068:       00050413                addi    s0,a0,0
   1006c:       00000493                addi    s1,zero,0
   10070:       00200913                addi    s2,zero,2
   10074:       02040063                beq     s0,zero,0x10094
   10078:       00897c63                bgeu    s2,s0,0x10090
   1007c:       fff40513                addi    a0,s0,-1
   10080:       fd5ff0ef                jal     ra,0x10054
   10084:       ffe40413                addi    s0,s0,-2
   10088:       00a484b3                add     s1,s1,a0
   1008c:       fe9ff06f                jal     zero,0x10074
   10090:       00100413                addi    s0,zero,1
   10094:       00940533                add     a0,s0,s1
   10098:       00c12083                lw      ra,12(sp)
   1009c:       00812403                lw      s0,8(sp)
   100a0:       00412483                lw      s1,4(sp)
   100a4:       00012903                lw      s2,0(sp)
   100a8:       01010113                addi    sp,sp,16
   100ac:       00008067                jalr    zero,0(ra)
   100b0:       fe010113                addi    sp,sp,-32    ; start address
   100b4:       00700513                addi    a0,zero,7    ; argument to fib()
   100b8:       00112e23                sw      ra,28(sp)    ; store ...
   100bc:       f99ff0ef                jal     ra,0x10054
   100c0:       00a12623                sw      a0,12(sp)
   100c4:       00c12783                lw      a5,12(sp)
   100c8:       01c12083                lw      ra,28(sp)
   100cc:       00f12623                sw      a5,12(sp)
   100d0:       02010113                addi    sp,sp,32
   100d4:       00008067                jalr    zero,0(ra)

*/
