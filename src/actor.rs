use std::collections::VecDeque;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_encrypt::{
    serialize::impls::BincodeSerializer, shared_key::SharedKey, traits::SerdeEncryptSharedKey,
    EncryptedMessage,
};

use crate::{Priority, Status};

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
}

impl<'b, T: DeserializeOwned> Actor<T> {
    pub fn new(initial_state: T, key: SharedKey) -> Self {
        Self {
            status: Status::Idle,
            msg: VecDeque::new(),
            priority: VecDeque::new(),
            state: initial_state,
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
}
