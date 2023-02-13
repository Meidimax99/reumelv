use super::{scheduler, state::*};
#[derive(PartialEq, Clone, Copy)]
pub struct Proc {
    pub idx: usize,
    pub id: usize,
}
impl Proc {
    pub unsafe fn get(&self) -> &'static mut ProcessData {
        if let Some(cur) = &mut scheduler::PROCS[self.idx] {
            if cur.id == self.id {
                return cur;
            }
            panic!(
                "Tried to access a user prog: {:?}, at: {}, but a different user prog was found: {:?}",
                self.id, self.idx, cur.id
            );
        }
        panic!(
            "Tried to access a not existing user prog: {:?}, at: {}",
            self.id, self.idx
        );
    }
    pub fn set_rdy(&self) {
        unsafe {
            self.get().state = State::Rdy;
        }
    }
    /// If blocked, returns the reason. Otherwise None.
    ///
    /// TODO Experimental
    pub fn is_blocked(&self, reason: Reason) -> bool {
        unsafe {
            if let State::Blocked(rsn, _) = self.get().state {
                return reason == rsn;
            }
            false
        }
    }

    pub fn is_blocked_of(&self, reason: Reason, number: usize) -> bool {
        unsafe { self.get().state == State::Blocked(reason, number) }
    }

    pub fn set_blocked(&self, reason: Reason, number: usize) {
        unsafe {
            self.get().state = State::Blocked(reason, number);
        }
    }

    pub fn increment_mepc(&self) {
        unsafe {
            self.get().mepc += 4;
        }
    }
    pub fn id(&self) -> usize {
        unsafe { self.get().id }
    }
    pub fn _sp(&self) -> usize {
        unsafe { self.get().sp }
    }
}

pub struct ProcessData {
    pub id: usize,
    pub init_proc_state: InitProcState,
    pub mepc: usize,
    pub sp: usize,
    pub state: State,
    pub priority: u8,
}

impl ProcessData {
    pub fn new(id: usize, init_proc_state: InitProcState) -> Self {
        ProcessData {
            id,
            init_proc_state,
            mepc: 0,
            sp: 0,
            state: State::Inactive,
            priority: 1,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct InitProcState {
    pub init_mepc: usize,
    pub pmp_idx: usize,
}

impl InitProcState {
    pub fn new(pmp_idx: usize) -> Self {
        InitProcState {
            init_mepc: 0,
            pmp_idx,
        }
    }
}
