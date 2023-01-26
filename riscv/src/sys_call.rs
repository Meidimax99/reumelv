pub enum SysCall {
    GetChar = 0,
    IpcSend = 1,
    IPCReceiver = 2,
    Print = 3,
    Yield = 23,
    Exit = 42,
    TaskNew = 68,
    LthreadExRegs = 69,
}
