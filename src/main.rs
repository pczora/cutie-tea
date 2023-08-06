#![allow(dead_code)]

use std::io;
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

pub mod packets;
pub mod types;

fn encode_variable_byte_int(value: u32) -> Vec<u8> {
    let mut x = value;
    let mut v: Vec<u8> = vec![];

    loop {
        let mut b: u8 = (x % 128).try_into().unwrap();
        x /= 128;
        if x > 0 {
            b |= 0b10000000;
        }
        v.push(b);
        if x == 0 {
            break;
        }
    }
    v
}


fn encode_string(value: &str) -> Vec<u8> {
    let len = (value.len() as u16).to_be_bytes();
    let mut bytes: Vec<u8> = vec![len[0], len[1]];
    for x in value.as_bytes() {
        bytes.push(*x)
    }

    bytes
}

#[derive(Debug)]
struct FixedHeader {
    packet_type: u8,
    remaining_length: u32,
}

impl FixedHeader {
    fn read(mut stream: TcpStream) -> io::Result<FixedHeader> {
        let mut buf_reader = BufReader::new(&mut stream);
        let mut header_bytes= [0u8; 32]; // Max size for fixed header is 6 bytes
        let bytes_read = buf_reader.read(&mut header_bytes[..])?;
        dbg!(bytes_read);

        let packet_type = (header_bytes[0] & 0xF0) >> 4; // Only the 4 most significant bits are used to determine the packet type

        Ok(FixedHeader{ packet_type, remaining_length: 0 })

    }
}


struct ConnectPacket<'a>
 {
    fh: FixedHeader,
    protocol_name: &'a str,
    protocol_version: u8,
    flags: u8,
    keepalive: u16,
    client_identifier: &'a str,
    will_topic: &'a str,
    will_message: &'a [u8],
    username: &'a str,
    password: &'a str,
}

impl ConnectPacket<'_> {
    fn new_simple() -> Self {
        ConnectPacket {
            fh: FixedHeader {
                packet_type: 1 << 4,
                remaining_length: 0,
            },
            protocol_name: "MQTT",
            protocol_version: 4u8,
            flags: 0,
            keepalive: 60,
            client_identifier: "cutie-tea1234",
            will_topic: "",
            will_message: "".as_bytes(),
            username: "",
            password: "",
        }
    }

    fn pack(mut self) -> Vec<u8> {
        let mut body = vec![];
        body.extend(encode_string(self.protocol_name));
        body.push(self.protocol_version);
        body.push(self.flags);
        body.push((self.keepalive >> 8) as u8);
        body.push(self.keepalive as u8);
        body.extend(encode_string(self.client_identifier));

        self.fh.remaining_length = body.len() as u32;
        let mut bytes = vec!(self.fh.packet_type );
        bytes.extend(encode_variable_byte_int(self.fh.remaining_length));
        bytes.extend(body);

        bytes
    }
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:1883").unwrap();
    let connect_packet = ConnectPacket::new_simple();

    
    //stream.write(connect_packet.pack().as_slice()).unwrap();
    stream.write_all(connect_packet.pack().as_slice());
    stream.flush();
    // sleep(Duration::from_secs(1));
    // let fh = FixedHeader::read(stream);
    // dbg!(fh.unwrap());

    let mut buf_reader = BufReader::new(&mut stream);
    let mut header_bytes= [0u8; 2]; // Max size for fixed header is 6 bytes
    let bytes_read = buf_reader.read(&mut header_bytes[..]).unwrap();
    dbg!(bytes_read);

    let packet_type = (header_bytes[0] & 0xF0) >> 4; // Only the 4 most significant bits are used to determine the packet type
    dbg!(packet_type);

    let remaining_bytes = if (header_bytes[1] >> 7) == 0 {
        header_bytes[1]
    } else {
        128
    } as u64;

    dbg!(remaining_bytes);

    let mut remaining_data = Vec::new();
    buf_reader.take(remaining_bytes).read_to_end(&mut remaining_data);
    dbg!(remaining_data);
}
