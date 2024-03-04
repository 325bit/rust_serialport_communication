use once_cell::sync::Lazy;

use std::vec::Vec;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, mpsc};
//use std::thread;
//use std::time::Duration;


pub struct MessageHeader{
    pub src_id: u32,
    pub dest_id: u32,
    pub primitive: u32,
    pub param: Vec<u32>,
}
impl MessageHeader {
    pub fn clone(msg: &MessageHeader) -> Self {
        MessageHeader {
            src_id: msg.src_id,
            dest_id: msg.dest_id,
            primitive: msg.primitive,
            param : msg.param.clone(),
        }
    }
}
// Define a struct to hold the senders with IDs
pub struct MailboxRegistry {
    senders: HashMap<u32, mpsc::Sender<MessageHeader>>,
}

impl MailboxRegistry {
    pub fn new() -> Self {
        MailboxRegistry {
            senders: HashMap::new(),
        }
    }

    pub fn add_mailbox(&mut self, id: u32, sender: mpsc::Sender<MessageHeader>) {
        self.senders.insert(id, sender);
    }

    pub fn get_mailbox_sender(&self, id: u32) -> Option<&mpsc::Sender<MessageHeader>> {
        self.senders.get(&id)
    }
}

pub static MAILBOX_MATRIX: Lazy<Arc<Mutex<MailboxRegistry>>> = Lazy::new(|| {
    Arc::new(Mutex::new(MailboxRegistry::new()))
});

pub struct MailboxSystem {
    //pub ch_matrix : MailboxRegistry,
}

impl MailboxSystem {

    pub fn create_mailbox_channel(id: u32) -> mpsc::Receiver<MessageHeader> {
        let (sender, receiver) = mpsc::channel();
        {
            MAILBOX_MATRIX.lock().unwrap().add_mailbox(id, sender);
        }
        println!("Created channel ID {}", id);
        return receiver;
    }

    pub fn mb_send_message(id:u32, msg:MessageHeader)
    {
        println!("try to send message from sender ID {}, trying to get lock()", id);
        let mailbox_registry = MAILBOX_MATRIX.lock().unwrap();
        if let Some(sender) = mailbox_registry.get_mailbox_sender(id) { //self.get_mailbox_channel_sender(id) {
            /*unlock MAILBOX_MATRIX here*/
            let sender_ref = sender.clone();
            drop(mailbox_registry);
            /*then send message*/
            sender_ref.send(msg).unwrap();
            println!("Sent message from sender ID {}", id);
        } else {
            println!("No sender found for ID {}", id);
        }
    }

}

pub fn print_message(msg:&MessageHeader)
{
    println!("messge src {}",msg.src_id);
    println!("messge dest {}",msg.dest_id);
    println!("messge primitive {}",msg.primitive);
    for element in &(msg.param) {
        println!("Element: {}", element);
    }
}
