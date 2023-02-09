#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State {
    Rdy,
    Inactive,
    _Blocked(Reason),
    Starting,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Reason {}
