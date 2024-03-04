
use lib_comm::serial::{Comm,SerialOperations};
use crate::protocol_common::{ProtocolOperations};

// Implement the SerialOperations trait for the Comm struct
pub struct ProtocolEmpty {
    pub protocol_name: String,
    pub comm: Comm,
}

pub fn empty_protocol_test(){
    println!("this is module function call of empty protcol\n")
}

impl ProtocolOperations for ProtocolEmpty {

    fn open(&mut self) -> Result<(), std::io::Error> {
        // Try to open the serial port with the specified settings
        match self.comm.open() {
            Ok(()) => Ok(()),
            Err(e) => {
                println!("Error opening empty protocol: {}", e);
                Err(e.into())
            }
        }
    }

    fn write(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        match self.comm.write(data) {
            Ok(()) => {
                Ok(())
            },
            Err(e) => {
                println!("Error sending to empty protocol: {}", e);
                Err(e.into())
            }
        }
    }

    fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, std::io::Error> {
        match self.comm.receive(buffer) {
            Ok(bytes_read) => Ok(bytes_read),
            Err(e) => {
                println!("Error reading from empty protocol: {}", e);
                Err(e.into())
            }
        }
    }
}