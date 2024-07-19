use std::io::ErrorKind;
use std::net::UdpSocket;
pub struct SensorServer {
    value: u16,
    udp_socket: UdpSocket,
}

impl SensorServer {
    pub fn new() -> Self {
        let udp_socket = UdpSocket::bind("0.0.0.0:5656").expect("Could not bind to address");

        udp_socket
            .set_nonblocking(true)
            .expect("Could not set non-blocking mode");

        SensorServer {
            value: 0,
            udp_socket: udp_socket,
        }
    }

    pub fn listen_for_new_data(&mut self) {
        let mut buf: [u8; 10] = [0; 10];
        match self.udp_socket.recv_from(&mut buf) {
            Ok((amt, _src)) => {
                let buf = std::str::from_utf8(&buf[..amt]).unwrap().trim();
                if let Ok(value) = buf.parse::<u16>() {
                    println!("Received value: {}", value);

                    self.value = value;
                }
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                // No data available right now, continue the loop
            }
            Err(e) => {
                eprintln!("Encountered an error: {}", e);
            }
        }
    }

    pub fn get_value(&self) -> u16 {
        self.value
    }
}
