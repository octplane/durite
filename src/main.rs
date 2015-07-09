
extern crate chrono;
extern crate docopt;

use std::process::Command;

use std::io::prelude::*;
use std::net::TcpStream;

use docopt::Docopt;

use std::str::FromStr;


// Write the Docopt usage string. dfrites ?
static USAGE: &'static str = "
Usage: durite -g GHOSTNAME -l HOSTNAME [-p PORT]
       durite (--help | -h)

Options:
    -h, --help     Show this screen.
    -l HOSTNAME    Hostname to advertise in graphite.
    -g GHOSTNAME   Graphite Hostname.
    -p PORT        Graphite port [default: 2003]
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

    let connect_string = format!("{}:{}", args.get_str("-g"), args.get_str("-p"));
    println!("Connect string is {}", connect_string);
    let my_hostname = args.get_str("-l");

	while true {
	    let dt = chrono::UTC::now();
	    let timestamp = dt.timestamp();
	    let o = disk_free();
		let stdout = String::from_utf8(o.stdout).
			ok().
			unwrap();

		let lines = stdout.split("\n");

	    match TcpStream::connect(&*connect_string) {
	    	Ok(s) => {
	    		let mut stream = s;
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
						// 	let o = statfs(mp.as_ptr(), &mut st);
						// 	println!("o{} {:?}", o, st.f_bfree);
						// }

                        // {:6E} 6 digit, lower exponential format
						send_content(&mut stream, my_hostname, format!("{}.available {:3.3}", disk, available), timestamp);
						send_content(&mut stream, my_hostname, format!("{}.all {}", disk, all), timestamp);
				 	}
				 }
	    	},
	    	Err(e) => println!("Unable to connect to {}: {:?}\nIgnoring this data point.", connect_string, e)
		}
	 	std::thread::sleep_ms(10000);
	}
}
