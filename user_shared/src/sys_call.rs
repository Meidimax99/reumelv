use core::arch::asm;
use riscv_utils as riscv;
use riscv_utils::SysCall;

/// Function to transmit the µ-kernel the type of the syscall, and two parameter.
/// It use the register a7, a0 and a1.
/// Switches to the kernel with an ecall
///
/// The return is the output from the µ-kernel
///
/// For example:
/// system_call(SysCall::GetChar, 0, 0);
unsafe fn system_call(syscall: SysCall, param_0: usize, param_1: usize) -> usize {
    let number = syscall as usize;
    riscv::write_function_reg!(
        number => "a7",
        param_0 => "a0",
        param_1 => "a1"
    );
    asm!("ecall");
    let output;
    riscv::read_function_reg!("a0" => output);
    return output;
}

/// Syscall to get a char from the user.
/// It returns a char.
pub fn get_char() -> Option<char> {
    unsafe {
        let res = system_call(SysCall::GetChar, 0, 0);
        if res == 0 {
            return None;
        }
        return Some(res as u8 as char);
    }
}
/// Syscall to exit a process
pub fn exit() {
    unsafe {
        system_call(SysCall::Exit, 0, 0);
    }
}
/// Syscall to give another process preemption
pub fn sys_yield() {
    unsafe {
        system_call(SysCall::Yield, 0, 0);
    }
}

pub fn task_new(mepc: usize) -> usize {
    unsafe {
        return system_call(SysCall::TaskNew, mepc, 0);
    }
}

pub fn sys_ipc_send(PID: usize, lenght: usize) {
    unsafe {
        system_call(SysCall::IpcSend, PID, lenght);
    }
}

pub fn sys_ipc_receive() -> usize {
    unsafe {
        let output;
        system_call(SysCall::IpcReceiver, pid, length);
        riscv::read_function_reg!("s0" => output);
        return output;
    }
}

pub fn sys_ipc_receive_all(pid: usize, length: usize) -> usize {
    unsafe {
        let output;
        system_call(SysCall::IpcReceiverAll, pid, length);
        riscv::read_function_reg!("s0" => output);
        return output;
    }
}
