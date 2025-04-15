use std::time::Duration;
use std::io::BufReader;
use std::io::BufRead;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    //let serial_port = serialport::new("COM6", 115200)
    let serial_port = serialport::new("COM3", 115200)
        .timeout(Duration::from_millis(4000))
        .open()
        .expect("Failed to open serial port");

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let target_adress = "127.0.0.1:8080";

    let mut reader = BufReader::new(serial_port);

    loop {
        let mut sound_str = String::new();
        reader.read_line(&mut sound_str).unwrap();

        let sound = match sound_str.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("{}", sound);
        socket.send_to(&sound.to_be_bytes(), target_adress)?;

    }
}