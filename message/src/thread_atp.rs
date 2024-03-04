use crate::message_lib::{MessageHeader,MailboxSystem,print_message};
use crate::thread::{MessageThreadOperations};

pub struct ATPTask {
    pub task_done: i32,
}

impl MessageThreadOperations for ATPTask {
    fn message_handler(&mut self, message: &MessageHeader) -> i32 {

        println!("ATP handle message tx_done {}:",self.task_done);
        print_message(message);

        if message.primitive == 2024 {
            self.task_done +=1;
        }
        else{
            let mut msg = MessageHeader::clone(message);
            msg.src_id = 1;
            msg.dest_id = 2;
            MailboxSystem::mb_send_message(msg.dest_id, msg);
        }

        if self.task_done >= 1 {
            let mut msg = MessageHeader::clone(message);
            msg.src_id = 1;
            msg.dest_id = 2;
            msg.primitive = 202412;
            MailboxSystem::mb_send_message(msg.dest_id, msg);
            return -1;
        }

        return 0;
    }
}