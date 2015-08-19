
extern crate chrono;
extern crate docopt;
extern crate riemann_client;

use riemann_client::Client;
use riemann_client::proto::Event;


use std::process::Command;

use std::io::prelude::*;
use std::net::TcpStream;

use docopt::Docopt;

use std::str::FromStr;


// Write the Docopt usage string. dfrites ?
static USAGE: &'static str = "
Usage: durite -l HOSTNAME [-g GHOSTNAME [-p PORT]] [-r RHOSTNAME [-o RPORT]]
       durite (--help | -h)

Options:
    -h, --help          Show this screen.
    -l HOSTNAME         Hostname to advertise.
    -g GHOSTNAME        Graphite Hostname.
    -r RHOSTNAME        Riemann Hostname.
    -o RPORT            Riemann port [default: 5555]
    -p PORT             Graphite port [default: 2003]
";

#[cfg(not(any(target_os="macos")))]
fn disk_free() -> std::process::Output {
	Command::new("df")
		.arg("--portability").output().unwrap_or_else(|e| { panic!("failed to execute process: {}", e) })
}

#[cfg(any(target_os="macos"))]
fn disk_free() -> std::process::Output {
	Command::new("df")
		.arg("-k").output().unwrap_or_else(|e| { panic!("failed to execute process: {}", e) })
}


fn send_content(stream: &mut std::net::TcpStream, hostname: &str, probe_and_value: String, timestamp: i64) {
	let content = format!("durite.{}.{} {}\n", hostname, probe_and_value, timestamp);
	let _ = stream.write(&content.as_bytes());
}

fn main() {

	// Parse argv and exit the program with an error message if it fails.
	let args = Docopt::new(USAGE)
		.and_then(|d| d.argv(std::env::args().into_iter()).parse())
		.unwrap_or_else(|e| e.exit());



    let graphite_connect_string: Option<String> = if args.get_str("-g") != "" {
        Some(format!("{}:{}", args.get_str("-g"), args.get_str("-p")))
    } else {
        None
    };


    let mut riemann_connection: Option<Client> = if args.get_str("-r") != "" {
        match Client::connect(&(args.get_str("-r"), u16::from_str(args.get_str("-o")).unwrap())) {
            Ok(c) => Some(c),
            Err(e) => {println!("Oups: {:?}", e); None}
        }
    } else {
        None
    };

    println!("Connect string is {:?}", graphite_connect_string);
    let my_hostname = args.get_str("-l");

	while true {
	    let dt = chrono::UTC::now();
	    let timestamp = dt.timestamp();
	    let o = disk_free();
		let stdout = String::from_utf8(o.stdout).
			ok().
			unwrap();

		let lines = stdout.split("\n");


        for line in lines {
            if line.starts_with("/") {
                let values: Vec<&str> = line.split(" ").filter(|e| {
                    e.len() != 0
                }).collect();

                let disk = values[0];
                // du reports kbytes

                let all: u64 = u64::from_str(values[1]).unwrap() / 1024;
                let available: u64 = u64::from_str(values[3]).unwrap() / 1024;
                if values.len() > 8 {
                }
                // let w = values[8].to_string();
                // let mut st: StatFs = Default::default();
                // let mp = CString::new(w.into_bytes()).unwrap();
                // unsafe {
                //  let o = statfs(mp.as_ptr(), &mut st);
                //  println!("o{} {:?}", o, st.f_bfree);
                // }

                // {:6E} 6 digit, lower exponential format

                match graphite_connect_string {
                    Some(ref connect_string) => {
                        let cs: &str = connect_string.as_ref();
                        match TcpStream::connect(cs) {
                            Ok(mut stream) => {
                                send_content(&mut stream, my_hostname, format!("{}.available {:3.3}", disk, available), timestamp);
                                send_content(&mut stream, my_hostname, format!("{}.all {}", disk, all), timestamp);
                            },
                            Err(e) => println!("Unable to connect to {}: {:?}\nIgnoring this data point.", connect_string, e)
                        }
                    },
                    None => {}
                }
                match riemann_connection {
                    Some(ref mut client) => {
                        client.event({
                                let mut event = Event::new();
                                event.set_service(format!("durite {} disk {} percent used", my_hostname, disk).to_string());
                                let percent_used: u64 = 100 - (available *100)/(all * 100);
                                println!("{}, {},  {} {}", percent_used, available, all, disk);
                                event.set_metric_d(percent_used as f64 / 100 as f64);
                                event
                            }).unwrap();
                    },
                    None => {}
                }
            }
        }
	 	std::thread::sleep_ms(10000);
	}
}
