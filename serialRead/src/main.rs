use serialport::SerialPort;
use std::time::Duration;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let mut serial_port = serialport::new("COM6:", 115200)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Failed to open serial port");

    let mut reader = BufReader::new(serial_port);
    let mut my_str = String::new();
    reader.read_line(&mut my_str).unwrap();

    println!("{}", my_str);
}