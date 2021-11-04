use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::process;

const MAX_BYTES_TO_READ_AT_ONCE: u64 = u64::pow(10, 7);

fn get_file_and_filesize(file_location: &str) -> Result<(File, &str, u64), io::Error> {
    let file_path = Path::new(file_location);
    let file_size = file_path.metadata()?.len();

    let file_absolute_path = file_path.canonicalize()?;
    let file_to_send = File::open(file_absolute_path)?;

    let file_name = file_path.file_name().unwrap().to_str().unwrap();

    Ok((file_to_send, file_name, file_size))
}

fn send_file_to_client(file_location: &str) -> Result<(), io::Error> {
    let (mut file_to_send, file_name, file_size) = get_file_and_filesize(file_location)?;

    let listener = TcpListener::bind("192.168.0.102:60000")?;
    let (mut tcp_socket, _socket_addr) = listener.accept()?;

    tcp_socket.write_fmt(format_args!("{}\n", file_name))?;

    // To receive "ACK" from client.
    let mut ack_buf = [0u8; 3];
    tcp_socket.read_exact(&mut ack_buf)?;

    let mut writer = BufWriter::with_capacity(MAX_BYTES_TO_READ_AT_ONCE as usize, &tcp_socket);
    let mut reader =
        BufReader::with_capacity(MAX_BYTES_TO_READ_AT_ONCE as usize, &mut file_to_send);

    let mut buffer = [0u8; MAX_BYTES_TO_READ_AT_ONCE as usize];

    let mut remaining_bytes_to_read = file_size;
    while remaining_bytes_to_read >= MAX_BYTES_TO_READ_AT_ONCE {
        reader.read_exact(&mut buffer)?;
        writer.write_all(&buffer)?;

        remaining_bytes_to_read -= MAX_BYTES_TO_READ_AT_ONCE;
    }

    if remaining_bytes_to_read != 0 {
        let mut last_buffer = Vec::<u8>::new();
        reader.read_to_end(&mut last_buffer)?;
        writer.write_all(&last_buffer)?;
    }

    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: server filename");
        process::exit(1);
    }

    if let Err(err) = send_file_to_client(&args[1]) {
        return Err(err.to_string());
    }

    Ok(())
}
