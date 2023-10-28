use serial2::SerialPort;
use std::time::{Duration, Instant};

pub fn upload(code: &str) {
    let code = code.as_bytes();

    let port = SerialPort::open("/dev/ttyACM0", 115200).unwrap();
    let magic: Vec<u8> = vec![0, 1, 2, 3, 4];

    port.write_all(&magic).unwrap();

    let mut buf = vec![0; 512];
    eprintln!("{}", port.read(&mut buf).unwrap());
    eprintln!("1. {}", String::from_utf8(buf).unwrap());

    let len = code.len() as u32;
    let len_buf = unsafe { std::slice::from_raw_parts(&len as *const u32 as *const u8, 32 / 8) };

    eprintln!("ll {:?}", len_buf);

    port.write_all(len_buf).unwrap();

    let mut buf = vec![0; 512];
    eprintln!("{}", port.read(&mut buf).unwrap());
    eprintln!("2.{}", String::from_utf8(buf).unwrap());

    port.write_all(code).unwrap();
    eprintln!("{}", code.len());

    let mut buf = vec![0; 512];
    eprintln!("{}", port.read(&mut buf).unwrap());
    eprintln!("3. {}", String::from_utf8(buf).unwrap());

    const MAX_WAIT_DURATION: Duration = Duration::from_secs(20);
    let start_time = Instant::now();
    loop {
        let mut buf = vec![0; 512];
        port.read(&mut buf).unwrap();
        if String::from_utf8(buf).unwrap().contains("ALL_GOOD") {
            break;
        }
        if Instant::now().duration_since(start_time) >= MAX_WAIT_DURATION {
            panic!("Too long!");
        }
    }
}
