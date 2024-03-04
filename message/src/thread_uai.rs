//use std::sync::{mpsc};

use crate::message_lib::{MessageHeader,MailboxSystem,print_message};
use crate::thread::{MessageThreadOperations};

pub struct UAITask {

}

impl MessageThreadOperations for UAITask {
    fn message_handler(&mut self, message: &MessageHeader) -> i32{
        println!("UAI handle message:");
        print_message(message);
        let mut msg = MessageHeader::clone(message);
        msg.src_id = 2;
        msg.dest_id = 3;
        MailboxSystem::mb_send_message(msg.dest_id, msg);
        if message.primitive == 202412 {
            return -1;
        }
        return 0;
    }
}