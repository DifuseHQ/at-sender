use clap::{App, Arg};
use serde::{Serialize};
use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::{io::Write, thread, time::Duration};
use serde_json;

#[derive(Serialize)]
struct Result {
    data: String,
}

fn main() {
    let matches = App::new("Serial Port Communication with AT commands")
        .version("0.1.0")
        .author("Hayzam <hayzam@difuse.io>")
        .about("Sends AT commands to a serial device and reads the response.")
        .arg(Arg::with_name("port")
             .long("port")
             .takes_value(true)
             .default_value("/dev/ttyUSB2")
             .help("The serial port to connect to"))
        .arg(Arg::with_name("baud")
             .long("baud")
             .takes_value(true)
             .default_value("115200")
             .help("The baud rate"))
        .arg(Arg::with_name("data")
             .long("data")
             .takes_value(true)
             .default_value("8")
             .help("The number of data bits"))
        .arg(Arg::with_name("stop")
             .long("stop")
             .takes_value(true)
             .default_value("1")
             .help("The number of stop bits"))
        .arg(Arg::with_name("cmd")
             .long("cmd")
             .takes_value(true)
             .default_value("AT")
             .help("The command to send to the serial device"))
        .arg(Arg::with_name("buf")
             .long("buf")
             .takes_value(true)
             .default_value("4")
             .help("The buffer size for reading from the serial device"))
        .get_matches();

        let port_name = matches.value_of("port").unwrap();
        let baud_rate = matches.value_of("baud").unwrap().parse::<u32>().expect("Invalid baud rate format");
        
        let data_bits = match matches.value_of("data").unwrap() {
            "8" => DataBits::Eight,
            "7" => DataBits::Seven,
            _ => panic!("Unsupported data bits"),
        };
        
        let stop_bits = match matches.value_of("stop").unwrap() {
            "1" => StopBits::One,
            "2" => StopBits::Two,
            _ => panic!("Unsupported stop bits"),
        };
        
        let command = matches.value_of("cmd").unwrap();
    
        let buf_size: usize = matches.value_of("buf").unwrap_or("4").parse().unwrap_or_else(|err| {
            eprintln!("Error parsing buffer size: {}", err);
            std::process::exit(1);
        });
    
        let port = serialport::new(port_name, baud_rate)
            .data_bits(data_bits)
            .flow_control(FlowControl::None)
            .parity(Parity::None)
            .stop_bits(stop_bits)
            .timeout(Duration::from_secs(1))
            .open()
            .expect("Failed to open port");
    
        let mut port = Box::new(port);
    
        port.write_all(command.as_bytes()).expect("Failed to write to port");
        port.write_all(b"\r").expect("Failed to write CR to port");
    
        thread::sleep(Duration::from_millis(1000));
    
        let mut buf = vec![0u8; buf_size];
        let n = port.read(&mut buf).expect("Failed to read from port");
        let result = Result {
            data: String::from_utf8_lossy(&buf[..n]).into_owned(),
        };
    
        let json_output = serde_json::to_string(&result).expect("Failed to serialize result");
        println!("{}", json_output);
}
