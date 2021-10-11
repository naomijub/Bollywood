use serde::{Deserialize, Serialize};

pub mod actor;
pub mod deadletter;

#[derive(Deserialize, Serialize)]
pub enum Status {
    Idle = 0,
    Working = 1,
    Stopped = 2,
    Killed = 3,
    Dead = 4,
    Error,
}

impl From<u8> for Status {
    fn from(status: u8) -> Self {
        match status {
            0 => Status::Idle,
            1 => Status::Working,
            2 => Status::Stopped,
            3 => Status::Killed,
            4 => Status::Dead,
            _ => Status::Error,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub enum Priority {
    Restart = 0,
    Kill = 1,
    Terminate = 2,
    Error,
}

impl From<u8> for Priority {
    fn from(priority: u8) -> Self {
        match priority {
            0 => Priority::Restart,
            1 => Priority::Kill,
            2 => Priority::Terminate,
            _ => Priority::Error,
        }
    }
}
