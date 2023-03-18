use core::mem;
use riscv_utils as riscv;

//This union represents a Message that can be sent over IPC
//The generic parameter T will be mapped onto the free registers that are used to transmit data with the ipc
#[repr(C)]
pub union Message<T: Copy> {
    pub content: T,
    mapping: [usize; 12],
}

const REGISTER_COUNT: usize = 12;
const MAX_SIZE: usize = mem::size_of::<usize>() * REGISTER_COUNT;

impl<T> Message<T>
where
    T: Copy,
{
    pub fn from_generic(data: T) -> Message<T> {
        if mem::size_of::<T>() > MAX_SIZE {
            panic!("Tried to create a IPC with to big contents!\n");
        }
        Message { content: data }
    }

    #[allow(unused_assignments)]
    pub fn from_registers() -> Message<T> {
        let mut mapping = [0usize; 12];
        unsafe {
            load_registers_into_array!(mapping ; "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11");
        }
        Message { mapping }
    }

    #[allow(unused_assignments)]
    pub fn write(&self) {
        unsafe {
            let mapping = self.mapping;
            write_array_to_registers!(mapping ; "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11");
        }
    }
}

/*
# Register Mapping Overview

https://vc.uni-bamberg.de/pluginfile.php/2209671/mod_resource/content/1/Elphinstone_HL-L4-79.pdf  -- Page 28

MIPS uses 28 Registers for IPC
- The saved registers are suitable for storing the message, since those are typically not changed across function calls
- RISCV64 has 12 64bit sized saved registers, 98 Bytes of Space

Additional Information of sent message in MIPS IPC
- snd descriptor        specifies if its a short ipc or if there is a buffered message, also specified if there is fpage mapping or simply a copying of the registers, can also specify whether virtual addresses (deceiving addresses) are to be used
- rcv descriptor
- timeouts
- dest id               Destination id, so for example the pid we are using
- waiting for id        specifies whether the calling thread is supposed to wait for any thread or a specific thread
- virtual sender id     "Deceiving address"

MIPS uses only one system call for ipc, if the call is a receiving ipc or a sending ipc is determined by the snd and rcv descriptors

Additional Information of received message in MIPS IPC
- real dest id
- msgdope + cc
- source id             sender id or virtual sender id, depending on whether the sender chose to deceive or not

MSGDOPE in MIPS Reg v0 -> return value register a0 in RISCV
- Describes the received message
*/
