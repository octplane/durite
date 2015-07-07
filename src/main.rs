extern crate riemann_client;
extern crate chrono;
extern crate docopt;

use std::process::Command;

use riemann_client::Client;
use riemann_client::proto::Event;

use std::io::prelude::*;
use std::net::TcpStream;

use docopt::Docopt;

use std::net::ToSocketAddrs;

// Write the Docopt usage string.
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


fn main() {

	// Parse argv and exit the program with an error message if it fails.
	let args = Docopt::new(USAGE)
		.and_then(|d| d.argv(std::env::args().into_iter()).parse())
		.unwrap_or_else(|e| e.exit());

	println!("{:?}", args);

    let connect_string = format!("{}:{}", args.get_str("-g"), args.get_str("-p"));
    println!("Connect string is {}", connect_string);
    let my_hostname = args.get_str("-l");



	// graphite.proto.melvil.io 2003
	// timestamp = 1436278152
	// Graphite format
	// local.random.diceroll 4 `date +%s`

	while true {
	    let dt = chrono::UTC::now();
	    let timestamp = dt.timestamp();
	    let o = disk_free();
		let stdout = String::from_utf8(o.stdout).
			ok().
			unwrap();

		let lines = stdout.split("\n");

	    let mut stream = TcpStream::connect(&*connect_string).unwrap();

		for line in lines {
			if line.starts_with("/") {
				let values: Vec<&str> = line.split(" ").filter(|e| {
					e.len() != 0
				}).collect();

				let disk = values[0];
				let all = values[2];
				let available = values[3];

				let content = format!("test.{}.{}.all {} {}\n", my_hostname, disk, all, timestamp);
				println!("{}", content);
				let _ = stream.write(&content.as_bytes());

				let content = format!("test.{}.{}.available {} {}\n", my_hostname, disk, available, timestamp);
				println!("{}", content);
				let _ = stream.write(&content.as_bytes());
		 	}
		}
	 	std::thread::sleep_ms(10000);
	}


    // let mut client = Client::connect(&("melvil.testing", 5555)).unwrap();
    // client.event({
    //     let mut event = Event::new();
    //     event.set_service("rust-riemann_client".to_string());
    //     event.set_state("ok".to_string());
    //     event.set_metric_d(128.128);
    //     event
    // }).unwrap();

    // client.event(riemann_client::Event {
    //     service: "rust-riemann_client",
    //     state: "ok",
    //     metric_d: 128.128
    //     ..Event::new()
    // }).unwrap()
}