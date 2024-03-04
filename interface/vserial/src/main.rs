use lib_comm::serial::{Comm};
use lib_comm::socket;
use lib_protocol::protocol_common::{ProtocolOperations};
use lib_protocol::tm;
use lib_protocol::empty::{ProtocolEmpty};
use std::thread;
use std::time::Duration;

fn thread_send(name: &str)
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
        //if count >= 10 { break;}
        thread::sleep(Duration::from_secs(1));
        count += 1;
    }

    println!("Thread {} end\n",name);
}

fn thread_recv(name: &str)
{
    let comm_recv = Comm {
        timeout: 0xFFFFFFFF,
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
            Ok(s) => println!("receiving {}", s),
            Err(_) => println!("Invalid UTF-8 sequence"),
        }
    }
    println!("Thread {} end\n",name);
}

fn main() {

    socket::mod_test();
    tm::tm_protocol_test();
    //empty::empty_protocol_test();
    thread::spawn(||{
        thread_recv("thread_recv");
    });
    thread::spawn(||{
        thread_send("thread_send");
    });
    loop {
        thread::sleep(Duration::from_secs(10));
        println!("Thread main alive\n");
    }
}