#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State {
    Rdy,
    Inactive,
    Blocked(Reason, usize),
    Starting,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Reason {
    SendingIpc,
    ReceiveIpc,
    ReceiveIpcAll,
}
