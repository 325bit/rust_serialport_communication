//use lib_comm::serial::{Comm,SerialOperations};

// Define a trait for the serial port operations
pub trait ProtocolOperations {
    fn open(&mut self) -> Result<(), std::io::Error>;
    fn write(&mut self, data: &[u8]) -> Result<(), std::io::Error>;
    fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, std::io::Error>;
}

