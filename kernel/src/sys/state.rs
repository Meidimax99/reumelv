#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    Rdy,
    Inactive,
    Blocked(Reason, usize),
    Starting,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Reason {
    SendingIpc,
    ReceiveIpc,
    ReceiveIpcAll,
}
