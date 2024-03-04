use std::vec::Vec;

use std::thread;
use std::time::Duration;

use lib_comm::serial::{Comm};
use lib_protocol::protocol_common::{ProtocolOperations};
use lib_protocol::empty::{ProtocolEmpty};

//mod message_lib;
use message::message_lib::{MailboxSystem,MessageHeader};
//mod thread;
use message::thread::{MessageTask};
use message::thread_atp::ATPTask;
use message::thread_uai::UAITask;
use message::thread_el1a::EL1ATask;

pub fn thread_send_message(mailbox_id:u32, src_id:u32, dest_id:u32, primitive:u32)
{
    let mut msg = MessageHeader {
        src_id : src_id,
        dest_id : dest_id,
        primitive : primitive,
        param : Vec::new(),
    };
    msg.param.push(primitive*2);
    MailboxSystem::mb_send_message(mailbox_id, msg);
}

fn thread_interface_recv(name: &str)
{
    let mut count = 0;
    let comm_recv = Comm {
        timeout: 1,
        port_name: "/dev/ttyVSP4".to_string(),
        port: None,
    };
    let mut empty_protocol_recv = ProtocolEmpty {
        protocol_name: "empty_protocol_r".to_string(),
        comm: comm_recv,
    };
    println!("Thread {} started\n",name);

    // Open the serial port
    match empty_protocol_recv.open() {
        Ok(p) => p,
        Err(e) => {
            println!("Error opening empty_protocol_recv: {}", e);
            return;
        }
    };

    loop {
        // Receive data from the serial port
        let mut buffer = [0; 256];
        if let Err(e) = empty_protocol_recv.receive(&mut buffer) {
            println!("Error receiving thread timeout: {}", e);
            break;
        }

        match std::str::from_utf8(&buffer) {
            Ok(s) => println!("{}", s),
            Err(_) => println!("Invalid UTF-8 sequence"),
        }
        thread_send_message(1,8,1,1970);
        count += 1;
        if count >= 10 {
            thread_send_message(1,8,1,2024);
            break;
        }
    }
    println!("Thread {} end\n",name);
}

fn thread_interface_send(name: &str)
{
    let mut count = 0;
    let comm_send = Comm {
        timeout: 0,
        port_name: "/dev/ttyVSP0".to_string(),
        port: None,
    };

    let mut empty_protocol_send = ProtocolEmpty {
        protocol_name: "empty_protocol_s".to_string(),
        comm: comm_send,
    };

    println!("Thread {} started\n",name);
    // Open the serial port
    match empty_protocol_send.open() {
        Ok(p) => p,
        Err(e) => {
            println!("Error opening empty_protocol_s: {}", e);
            return;
        }
    };

    loop {
        // Write data to the serial port
        let sending_str = format!("Hello World {}\n",count);

        if let Err(e) = empty_protocol_send.write(sending_str.as_bytes()) { //b"Hello World!\n") {
            println!("Error writing to empty_protocol_s: {}", e);
            return;
        }
        if count >= 10 { break;}
        thread::sleep(Duration::from_secs(1));
        count += 1;
    }

    println!("Thread {} end\n",name);
}

fn main() {
    let thread_atp = thread::spawn( move || {
            println!("start Receiver");
            let mut atp_task = MessageTask::new(ATPTask{task_done:0,},1);

            atp_task.thread_receive_message();
    });

    let thread_uai = thread::spawn( move || {
        println!("start Receiver");
        let mut uai_task = MessageTask::new(UAITask{},2);

        uai_task.thread_receive_message();
    });

    let thread_el1a = thread::spawn( move || {
        println!("start Receiver");
        let mut el1a_task = MessageTask::new(EL1ATask{},3);

        el1a_task.thread_receive_message();
    });

    thread::sleep(Duration::from_secs(1));

    let thread_send2 = thread::spawn( || {
        thread_interface_recv("interface_recv");
    });
    let thread_send1 = thread::spawn( move || {
        thread_interface_send("interface_send");
    });
    match thread_send1.join() {
        Ok(_) => println!("The send thread1 has finished."),
        Err(_) => println!("An error occurred while joining the thread"),
    }
    match thread_send2.join() {
        Ok(_) => println!("The send thread2 has finished."),
        Err(_) => println!("An error occurred while joining the thread"),
    }
    match thread_atp.join() {
        Ok(_) => println!("The recv thread has finished."),
        Err(_) => println!("An error occurred while joining the thread:"),
    }
    match thread_uai.join() {
        Ok(_) => println!("The recv thread has finished."),
        Err(_) => println!("An error occurred while joining the thread:"),
    }
    match thread_el1a.join() {
        Ok(_) => println!("The recv thread has finished."),
        Err(_) => println!("An error occurred while joining the thread:"),
    }

}
