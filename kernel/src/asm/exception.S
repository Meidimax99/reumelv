.global exception
.global exception_handler
.align 4
.set REG_SIZE, 8
exception:
        //More info https://web.eecs.utk.edu/~smarz1/courses/ece356/notes/assembly/
        // make room to save registers.
        addi sp, sp, -256       //TODO only 31 values are stored into stack, but space for 32 is reserved? This results in inconsistencies in stack.rs
                                //This does not result in loss of data since    x0 is hardwired to 0 anyways
        // save the registers.
        //Infos on register https://en.wikichip.org/wiki/risc-v/registers


        sd ra, 0(sp)
        sd sp, 1*REG_SIZE(sp)
        sd gp, 2*REG_SIZE(sp)
        sd tp, 3*REG_SIZE(sp)
        sd t0, 4*REG_SIZE(sp)
        sd t1, 5*REG_SIZE(sp)
        sd t2, 6*REG_SIZE(sp)
        sd s0, 7*REG_SIZE(sp)
        sd s1, 8*REG_SIZE(sp)
        sd a0, 9*REG_SIZE(sp)
        sd a1, 10*REG_SIZE(sp)
        sd a2, 11*REG_SIZE(sp)
        sd a3, 12*REG_SIZE(sp)
        sd a4, 13*REG_SIZE(sp)
        sd a5, 14*REG_SIZE(sp)
        sd a6, 15*REG_SIZE(sp)
        sd a7, 16*REG_SIZE(sp)
        sd s2, 17*REG_SIZE(sp)
        sd s3, 18*REG_SIZE(sp)
        sd s4, 19*REG_SIZE(sp)
        sd s5, 20*REG_SIZE(sp)
        sd s6, 21*REG_SIZE(sp)
        sd s7, 22*REG_SIZE(sp)
        sd s8, 23*REG_SIZE(sp)
        sd s9, 24*REG_SIZE(sp)
        sd s10, 25*REG_SIZE(sp)
        sd s11, 26*REG_SIZE(sp)
        sd t3, 27*REG_SIZE(sp)
        sd t4, 28*REG_SIZE(sp)
        sd t5, 29*REG_SIZE(sp)
        sd t6, 30*REG_SIZE(sp)

        sd zero, 31*REG_SIZE(sp)        //Added to provide all registers in the memory snapshot of the registers and fit the alignment

        //More info https://riscv.org/wp-content/uploads/2017/05/riscv-privileged-v1.10.pdf
        csrr a0, mepc           //Read control register mepc (Machine exception program counter) into a0 (function argument 0/ return value 0)
        csrr a1, mcause         //Read control register mcause (Machine control register) into a1 (function argument 1/ return value 1)   
        mv a2, sp               // Copy stack pointer int a2 as function argument 2
        la sp, _stack_end       //TODO Where does "_stack_end" come from

        // call the C trap handler in exception_handler.rs
        call exception_handler

        //Restore Stack Pointer, the stackpointer is returned from the exception_handler function
        mv sp, a0

        // restore registers.
        ld ra, 0(sp)
        ld sp, 1*REG_SIZE(sp)
        ld gp, 2*REG_SIZE(sp)
        ld tp, 3*REG_SIZE(sp)
        ld t0, 4*REG_SIZE(sp)
        ld t1, 5*REG_SIZE(sp)
        ld t2, 6*REG_SIZE(sp)
        ld s0, 7*REG_SIZE(sp)
        ld s1, 8*REG_SIZE(sp)
        ld a0, 9*REG_SIZE(sp)
        ld a1, 10*REG_SIZE(sp)
        ld a2, 11*REG_SIZE(sp)
        ld a3, 12*REG_SIZE(sp)
        ld a4, 13*REG_SIZE(sp)
        ld a5, 14*REG_SIZE(sp)
        ld a6, 15*REG_SIZE(sp)
        ld a7, 16*REG_SIZE(sp)
        ld s2, 17*REG_SIZE(sp)
        ld s3, 18*REG_SIZE(sp)
        ld s4, 19*REG_SIZE(sp)
        ld s5, 20*REG_SIZE(sp)
        ld s6, 21*REG_SIZE(sp)
        ld s7, 22*REG_SIZE(sp)
        ld s8, 23*REG_SIZE(sp)
        ld s9, 24*REG_SIZE(sp)
        ld s10, 25*REG_SIZE(sp)
        ld s11, 26*REG_SIZE(sp)
        ld t3, 27*REG_SIZE(sp)
        ld t4, 28*REG_SIZE(sp)
        ld t5, 29*REG_SIZE(sp)
        ld t6, 30*REG_SIZE(sp)
        

        addi sp, sp, 256 //clear saved registers from stack

        // return to whatever we were doing in the kernel.
        mret
