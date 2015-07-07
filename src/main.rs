extern crate riemann_client;
extern crate chrono;
extern crate docopt;

use std::process::Command;

use riemann_client::Client;
use riemann_client::proto::Event;

use std::io::prelude::*;
use std::net::TcpStream;

use docopt::Docopt;


extern crate libc;
use libc::*;
use std::ffi::CString;
use std::default::Default;

// #define MFSTYPENAMELEN  16  length of fs type name including null 
// #define MAXPATHLEN      1024
// #define MNAMELEN        MAXPATHLEN
   // struct statfs { /* when _DARWIN_FEATURE_64_BIT_INODE is defined */
   //       uint32_t    f_bsize;        /* fundamental file system block size */
   //       int32_t     f_iosize;       /* optimal transfer block size */
   //       uint64_t    f_blocks;       /* total data blocks in file system */
   //       uint64_t    f_bfree;        /* free blocks in fs */
   //       uint64_t    f_bavail;       /* free blocks avail to non-superuser */
   //       uint64_t    f_files;        /* total file nodes in file system */
   //       uint64_t    f_ffree;        /* free file nodes in fs */
   //       fsid_t      f_fsid;         /* file system id */
   //       uid_t       f_owner;        /* user that mounted the filesystem */
   //       uint32_t    f_type;         /* type of filesystem */
   //       uint32_t    f_flags;        /* copy of mount exported flags */
   //       uint32_t    f_fssubtype;    /* fs sub-type (flavor) */
   //       char        f_fstypename[MFSTYPENAMELEN];   /* fs type name */
   //       char        f_mntonname[MAXPATHLEN];        /* directory on which mounted */
   //       char        f_mntfromname[MAXPATHLEN];      /* mounted filesystem */
   //       uint32_t    f_reserved[8];  /* For future use */
   //   };


type uid = uint32_t;


#[repr(C)]
struct StatFs {
	f_bsize: uint32_t,
	f_iosize: int32_t,
	f_blocks: uint64_t,
	f_bfree:  uint64_t,
	f_bavail: uint64_t,
	f_files:  uint64_t,
	f_ffree:  uint64_t,
	f_fsid:	  [uint32_t; 2],
	f_owner:  uid,
	f_type:	  uint32_t,
	f_flags:  uint32_t,
	f_fstypename: 	[c_char; 16],
	f_mntonname: 	[c_char; 1024],
	f_mntfromname: 	[c_char; 1024],
	f_reserved: 	[uint32_t; 8],
}

impl Default for StatFs {
	fn default() -> StatFs {
		StatFs{
			f_bsize: 0,
			f_iosize: 0,
			f_blocks: 0,
			f_bfree: 0,
			f_bavail: 0,
			f_files: 0,
			f_ffree: 0,
			f_fsid: [0; 2],
			f_owner: 0,
			f_type: 0,
			f_flags: 0,
			f_fstypename: [0; 16],
			f_mntonname:  [0; 1024],
			f_mntfromname:[0; 1024],
			f_reserved: [0; 8]
		}
	}
}

extern {
	fn statfs(path: *const c_char, stafs: *mut StatFs) -> size_t;
	// statfs(const char *path, struct statfs *buf);
}

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


fn main() {

	// Parse argv and exit the program with an error message if it fails.
	let args = Docopt::new(USAGE)
		.and_then(|d| d.argv(std::env::args().into_iter()).parse())
		.unwrap_or_else(|e| e.exit());

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

	    match TcpStream::connect(&*connect_string) {
	    	Ok(s) => {
	    		let mut stream = s;
				for line in lines {
					if line.starts_with("/") {
						let values: Vec<&str> = line.split(" ").filter(|e| {
							e.len() != 0
						}).collect();

						let disk = values[0];
						let all = values[2];
						let available = values[3];
						let w = values[8].to_string();
						let mut st: StatFs = Default::default();
						let mp = CString::new(w.into_bytes()).unwrap();
						unsafe {
							let o = statfs(mp.as_ptr(), &mut st);
							println!("o{}", o);
						}


						let content = format!("test.{}.{}.all {} {}\n", my_hostname, disk, all, timestamp);
						println!("{}", content);
						let _ = stream.write(&content.as_bytes());

						let content = format!("test.{}.{}.available {} {}\n", my_hostname, disk, available, timestamp);
						println!("{}", content);
						let _ = stream.write(&content.as_bytes());
				 	}
				 }
	    	},
	    	Err(e) => println!("Unable to connect to {}: {:?}\nIgnoring this data point.", connect_string, e)
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