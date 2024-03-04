//use std::sync::{mpsc};

use crate::message_lib::{MessageHeader,print_message};
use crate::thread::{MessageThreadOperations};

pub struct EL1ATask {

}

impl MessageThreadOperations for EL1ATask {
    fn message_handler(&mut self, message: &MessageHeader) -> i32{
        println!("EL1ATask handle message:");
        print_message(message);
        if message.primitive == 202412 {
            return -1;
        }
        return 0;
    }
}