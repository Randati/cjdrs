#![allow(unstable)]

extern crate cjdrs;
extern crate mio;
extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;

#[cfg(not(test))] use std::{os, io};
#[cfg(not(test))] use docopt::Docopt;
#[cfg(not(test))] use cjdrs::CjdrsError;
#[cfg(not(test))] use cjdrs::CjdrsResult;
#[cfg(not(test))] use cjdrs::Config;
#[cfg(not(test))] use cjdrs::EventHandler;
#[cfg(not(test))] use cjdrs::interface::{self, NetInterface};
#[cfg(not(test))] use cjdrs::Router;
#[cfg(not(test))] use cjdrs::{PrivateKey, PrivateIdentity};


static USAGE: &'static str = "
Usage: cjdrs --help
       cjdrs init [--cfg=<file>]
       cjdrs run [--cfg=<file>]

Options:
  -h, --help      Show this message.
  --cfg=<file>    Configuration file [default: cjdrs.conf]

1. Run 'cjdrs init' to generate a configuration file.
2. Edit the configuration file as needed.
2. Run 'cjdrs run' to start cjdrs.

Configuration file defaults to 'cjdrs.conf' if not given.
";

#[derive(RustcDecodable, Show)]
struct Args {
	cmd_init: bool,
	cmd_run: bool,
	flag_cfg: String,
}

#[cfg(not(test))]
fn main() {
	if let Err(e) = choose_command() {
		os::set_exit_status(1);
	
		let mut stderr = io::stdio::stderr();
		writeln!(&mut stderr, "Error: {:?}", e).unwrap();
	}
}


#[cfg(not(test))]
fn choose_command() -> CjdrsResult<()> {
	let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
	let config_path = Path::new(args.flag_cfg);

	cjdrs::init();

	if args.cmd_init {
		init_config(&config_path)
	} else {
		assert!(args.cmd_run);
		let config = try!(Config::load(&config_path));
		run_cjdrs(&config)
	}
}


#[cfg(not(test))]
fn init_config(config_path: &Path) -> CjdrsResult<()> {
	let identity = PrivateIdentity::generate();
		
	let config = Config::get_default(&identity);
	try!(config.write(config_path));

	println!("Created a new configuration file '{}'", config_path.display());
	println!("Public key: {}", identity.public_key);
	println!("Address:    {}", identity.address);

	Ok(())
}


#[cfg(not(test))]
fn run_cjdrs(config: &Config) -> CjdrsResult<()> {
	// Create identity
	let my_identity = {
		let private_key = try!(PrivateKey::from_string(config.privateKey.as_slice()));
		try!(PrivateIdentity::from_private_key(&private_key).ok_or(
			CjdrsError::NoAddressForPrivateKey(private_key)))
	};

	println!("Public key: {}", my_identity.public_key);
	println!("Address:    {}", my_identity.address);
	

	// Turn on interfaces
	let tun_interface = interface::Tun::new(
		config.tunDevice.as_slice(),
		&my_identity.address);
	println!("Opened tun device '{}'", tun_interface.get_name());

	let udp_interface = try!(interface::Udp::create(config.udpBind.as_slice()));

	let interfaces: Vec<Box<NetInterface>> = vec![
		Box::new(tun_interface) as Box<NetInterface>,
		Box::new(udp_interface) as Box<NetInterface>,
	];


	let router = Router::new(&my_identity.address);


	// Start up the event loop
	let mut mio_loop: mio::EventLoop<usize, ()> = try!(mio::EventLoop::new());
	
	let event_handler = EventHandler::new(
		my_identity,
		interfaces,
		router);

	try!(event_handler.register_handlers(&mut mio_loop));
	try!(mio_loop.run(event_handler));

	Ok(())
}
