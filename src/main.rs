#![feature(phase)]

extern crate cjdrs;
extern crate mio;
extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;
#[phase(plugin)] extern crate docopt_macros;

#[cfg(not(test))] use docopt::Docopt;
#[cfg(not(test))] use cjdrs::Config;
#[cfg(not(test))] use cjdrs::{PrivateKey, PrivateIdentity};
#[cfg(not(test))] use cjdrs::interface;
#[cfg(not(test))] use cjdrs::Router;
#[cfg(not(test))] use cjdrs::EventHandler;

docopt!(Args deriving Show, "
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
");



#[cfg(not(test))]
fn main() {
	let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
	let config_path = Path::new(args.flag_cfg);

	cjdrs::init();



	// Generate new configuraion file
	if args.cmd_init {
		let identity = PrivateIdentity::generate();
		
		let config = Config::get_default(&identity);
		config.write(&config_path).unwrap();

		println!("Created new configuration file '{}'", config_path.display());
		println!("Public key: {}", identity.public_key.as_string());
		println!("Address:    {}", identity.address);
		return;
	}

	// Otherwise continue running
	assert!(args.cmd_run);


	let config = Config::load(&config_path);
	let identity = {
		let private_key = PrivateKey::from_string(config.privateKey.as_slice()).unwrap();
		PrivateIdentity::from_private_key(&private_key).unwrap()
	};

	println!("Public key:  {}", identity.public_key.as_string());
	println!("Address:     {}", identity.address);
	

	let tun_interface = interface::Tun::new(
		config.tunDevice.as_slice(),
		&identity.address);
	println!("Opened tun device '{}'", tun_interface.get_name());

	
	let udp_interface = interface::Udp::create(config.udpBind.as_slice());


	let router = Router::new(&identity.address);


	let mut mio_loop: mio::EventLoop<uint, ()> = mio::EventLoop::new().unwrap();
	let event_handler = EventHandler::new(&mut mio_loop,
		identity,
		tun_interface,
		udp_interface,
		router).ok().expect("Couldn't create the event handler");
	
	if let Err(e) = mio_loop.run(event_handler) {
		panic!("Error while running event loop: {}", e.error);
	}
}
