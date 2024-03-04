use serialport::{SerialPort, SerialPortSettings};
use std::time::Duration;

// Define a trait for the serial port operations
pub trait SerialOperations {
    fn open(&mut self) -> Result<(), std::io::Error>;
    fn write(&mut self, data: &[u8]) -> Result<(), std::io::Error>;
    fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, std::io::Error>;
}

// Implement the SerialOperations trait for the Comm struct
pub struct Comm {
    pub timeout: u64,
    pub port_name: String,
    pub port: Option<Box<dyn SerialPort>>,
}

impl SerialOperations for Comm {
    fn open(&mut self) -> Result<(), std::io::Error> {
        // Create the serial port settings
        let settings = SerialPortSettings {
            baud_rate: 9600,
            ..Default::default()
        };

        // Try to open the serial port with the specified settings
        match serialport::open_with_settings(&self.port_name, &settings) {
            Ok(p) => {
                self.port = Some(p);
                if self.timeout > 0 {
                    let mut comm_timeout = self.port.as_mut().unwrap().timeout(); /*directly use port of Comm*/
                    println!("Duration is seconds: {:.2?}\n",comm_timeout.as_secs_f32());
                    comm_timeout = Duration::from_secs(self.timeout);
                    let _ = self.port.as_mut().unwrap().set_timeout(comm_timeout);
                }
                Ok(())
            },
            Err(e) => {
                println!("Error opening serial port: {}", e);
                Err(e.into())
            }
        }
    }

    fn write(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        if let Some(port) = &mut self.port {
            port.write(data)?;
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Serial port not open",
            ))
        }
    }

    fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, std::io::Error> {
        if let Some(port) = &mut self.port {
            //Ok(port.read(buffer)?)
            //let timeout = Duration::from_secs(10);
            match port.read(buffer) {
                Ok(bytes_read) => Ok(bytes_read),
                Err(e) => {
                    println!("Error reading from serial port: {}", e);
                    Err(e.into())
                }
            }
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Serial port not open",
            ))
        }
    }
}
