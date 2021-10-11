use std::collections::VecDeque;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_encrypt::{
    serialize::impls::BincodeSerializer, shared_key::SharedKey, traits::SerdeEncryptSharedKey,
    EncryptedMessage,
};

use crate::{deadletter::DeadLetter, Priority, Status};

#[derive(Deserialize, Serialize)]
pub struct Message(String);

impl SerdeEncryptSharedKey for Message {
    type S = BincodeSerializer<Self>;
}

pub struct Actor<T> {
    status: Status,
    msg: VecDeque<T>,
    priority: VecDeque<Priority>,
    state: T,
    key: SharedKey,
    letters: DeadLetter<T>,
}

impl<'b, T: DeserializeOwned> Actor<T> {
    pub fn new(initial_state: T, key: SharedKey) -> Self {
        Self {
            status: Status::Idle,
            msg: VecDeque::new(),
            priority: VecDeque::new(),
            state: initial_state,
            letters: DeadLetter::new(),
            key,
        }
    }

    pub fn post_priority(&mut self, priority: Priority) {
        self.priority.push_back(priority);
    }

    pub fn post_msg(&mut self, encrypted_msg: String) {
        let encrypted_message =
            EncryptedMessage::deserialize(encrypted_msg.as_bytes().to_vec()).unwrap();
        let msg = Message::decrypt_owned(&encrypted_message, &self.key).unwrap();
        let msg1 = msg.0;
        let msg2 = ron::from_str::<T>(&msg1).unwrap();
        self.msg.push_back(msg2);
    }

    pub fn stop(&mut self) {
        self.status = Status::Stopped
    }

    pub fn execute(mut self) {
        if !self.priority.is_empty() {
            self.status = Status::Working;
            self.execute_priority();
        } else if !self.msg.is_empty() {
            self.status = Status::Working;
            self.execute_msg();
        } else {
            self.status = Status::Idle;
        }
    }

    pub fn execute_msg(mut self) {
        let msg = self.msg.pop_front().unwrap();
        self.state = msg;
        self.execute();
    }

    pub fn execute_priority(mut self) {
        let priority = self.priority.pop_front().unwrap();
        match priority {
            Priority::Restart => {
                self.status = Status::Working;
                for msg in self.msg {
                    self.letters.post(msg);
                }

                self.priority = VecDeque::new();
                self.msg = VecDeque::new();
                self.status = Status::Idle;
            }
            Priority::Kill => {
                drop(self);
            }
            Priority::Terminate => {
                for msg in self.msg {
                    self.letters.post(msg);
                }
                self.status = Status::Stopped;
            }
            Priority::Error => panic!("Unexpected error"),
        }
    }
}
