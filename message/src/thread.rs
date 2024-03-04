use std::sync::{mpsc};
//use std::io::Error;

//mod message_lib;
use crate::message_lib::{MailboxSystem,MessageHeader};

// Define a trait for the serial port operations
pub trait MessageThreadOperations {
    fn message_handler(&mut self, message: &MessageHeader) -> i32;
}

// Implement the SerialOperations trait for the Comm struct
pub struct MessageTask<H>
where
H : MessageThreadOperations + 'static {
    mailbox: mpsc::Receiver<MessageHeader>,
    handler: H,
}

impl<H> MessageTask<H>
where
H : MessageThreadOperations + 'static {
    pub fn new(handler:H, id: u32) -> Self {
        MessageTask {
            mailbox: MailboxSystem::create_mailbox_channel(id),
            handler: handler,
        }
    }
    pub fn thread_receive_message(&mut self) {
        loop{
            match self.mailbox.recv() {
                Ok(msg) => {
                    let loop_done = self.handler.message_handler(&msg);
                    if loop_done < 0 {
                        break;
                    }
                },
                Err(_) => {
                    println!("Receiver fail");
                }
            }
        }
    }
}
/*
impl MessageThreadOperations for MessageTask {
    fn message_handler(&mut self, message: &MessageHeader){
        print_message(message);
    }
}*/