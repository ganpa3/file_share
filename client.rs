use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use std::str;

const MAX_BYTES_TO_READ_AT_ONCE: u64 = u64::pow(10, 7);
const DELIMITER: u8 = 10u8;

fn receive_file() -> Result<(), io::Error> {
    let mut stream = TcpStream::connect("192.168.0.102:60000")?;
    let mut file_name = String::new();
    let mut file_name_buf = [0u8; 10];

    loop {
        let bytes_read = stream.read(&mut file_name_buf)?;
        let last_index = if file_name_buf[bytes_read - 1] == DELIMITER {
            bytes_read - 2
        } else {
            bytes_read - 1
        };

        match str::from_utf8(&file_name_buf[..last_index + 1]) {
            Ok(s) => file_name.push_str(s),
            Err(_err) => {
                file_name = String::from("received_file");
                break;
            }
        };

        if file_name_buf[bytes_read - 1] == DELIMITER {
            break;
        }
    }

    stream.write_fmt(format_args!("ACK"))?;

    let mut reader = BufReader::with_capacity(MAX_BYTES_TO_READ_AT_ONCE as usize, &mut stream);

    let file_to_receive = File::create(&file_name).unwrap();
    let mut writer = BufWriter::with_capacity(MAX_BYTES_TO_READ_AT_ONCE as usize, &file_to_receive);

    let mut buf = [0u8; MAX_BYTES_TO_READ_AT_ONCE as usize];

    loop {
        match reader.read(&mut buf) {
            Ok(0) => {
                break;
            }
            Ok(n) => {
                writer.write(&mut buf[..n])?;
            }
            Err(e) => {
                dbg!(e);
                break;
            }
        }
    }

    println!("File received: {}\n", file_name);
    Ok(())
}

fn main() -> Result<(), String> {
    if let Err(err) = receive_file() {
        return Err(err.to_string());
    }

    Ok(())
}
