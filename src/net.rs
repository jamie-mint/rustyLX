use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;


static NUM_BYTES_PER_PIXEL: u32 = 3;
static EXTRA_BYTES: u32 = 2;

/////////////////////////////

static DEFAULT_PORT: i32 = 7890;

static HEADER_LEN: i32 = 4;

static OFFSET_CHANNEL: usize = 0;
static OFFSET_COMMAND: usize = 1;
static OFFSET_DATA_LEN_MSB: usize = 2;
static OFFSET_DATA_LEN_LSB: usize = 3;
static OFFSET_DATA: usize = 4;

static  CHANNEL_BROADCAST: u8 = 0;

static  COMMAND_SET_PIXEL_COLORS: u8 = 0;
static  COMMAND_SYSTEM_EXCLUSIVE: u8 = 0xff;

struct OpcDatagram {
    data_length: u32,
    total_buffer_size: u32,
    buffer: Vec<u8>,
}

impl OpcDatagram {
    pub fn new (channel: u8, num_pixels: u32) -> OpcDatagram {
        let data_length = num_pixels*NUM_BYTES_PER_PIXEL;
        let total_buffer_size = data_length + EXTRA_BYTES;
        let buffer = vec![0xa2; total_buffer_size as usize];

        let mut constructed = OpcDatagram {
            data_length,
            total_buffer_size,
            buffer,
        };

        constructed.buffer[OFFSET_CHANNEL] = channel;
        constructed.buffer[OFFSET_COMMAND] = COMMAND_SET_PIXEL_COLORS;
        constructed.buffer[OFFSET_DATA_LEN_MSB] = ((data_length >> 8) & 0xff) as u8;
        constructed.buffer[OFFSET_DATA_LEN_LSB] = (data_length & 0xff) as u8;

        constructed
    }

    pub fn debug (&mut self) {
        println!("hi debug")
    }

    pub fn test_fill_magenta(&mut self) {
        let mut slice = &mut self.buffer[OFFSET_DATA ..];
    }

    pub fn send_opc(&mut self) {
        match TcpStream::connect(format!("{}{}", "localhost:", DEFAULT_PORT)) {
            Ok(mut stream) => {
                println!("Successfully connected to server in port {}", DEFAULT_PORT);

                for x in 0..5 {
                    stream.write(self.buffer.as_ref()).unwrap();
                    stream.flush().unwrap();
                }
                // let msg = b"Hello!";
                //
                // stream.write(msg).unwrap();
                // println!("Sent Hello, awaiting reply...");
                //
                // let mut data = [0 as u8; 6]; // using 6 byte buffer
                // match stream.read_exact(&mut data) {
                //     Ok(_) => {
                //         if &data == msg {
                //             println!("Reply is ok!");
                //         } else {
                //             let text = from_utf8(&data).unwrap();
                //             println!("Unexpected reply: {}", text);
                //         }
                //     },
                //     Err(e) => {
                //         println!("Failed to receive data: {}", e);
                //     }
                // }
            },
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
        println!("Terminated.");
    }
}


fn net_main() {
    println!("main");
}


#[cfg(test)]
mod tests {
    use crate::net::OpcDatagram;

    #[test]
    fn write_pix_buffer() {
        assert_eq!(2 + 2, 4);
    }


    #[test]
    fn test_initialize_buffer() {
        let mut migram = OpcDatagram::new( 0, 423);
        migram.debug();
    }

    #[test]
    fn test_send_to_zestyping_opc_server() {
        let mut migram = OpcDatagram::new( 0, 4097);
        migram.test_fill_magenta();
        migram.send_opc();
    }
}
