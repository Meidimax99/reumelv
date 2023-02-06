pub enum SysCall {
    GetChar = 0,
    IpcSend = 1,
    IpcReceiver = 2,
    IpcReceiverAll = 3,
    Print = 4,
    Yield = 23,
    Exit = 42,
    TaskNew = 68,
    LthreadExRegs = 69,
}
