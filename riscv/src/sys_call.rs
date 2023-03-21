pub enum SysCall {
    GetChar = 0,
    Print = 1,
    Yield = 23,
    Exit = 42,
    TaskNew = 68,
    LthreadExRegs = 69,
}
