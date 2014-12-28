extern crate cjdrs;
extern crate mio;

#[cfg(not(test))] use std::rand::OsRng;
#[cfg(not(test))] use cjdrs::{PrivateKey, PrivateIdentity};
#[cfg(not(test))] use cjdrs::interface;
#[cfg(not(test))] use cjdrs::Router;
#[cfg(not(test))] use cjdrs::EventHandler;

#[cfg(not(test))]
fn main() {
	println!("Hello, cjdns!");

	let mut rng = match OsRng::new() {
		Err(e) => panic!("No random number generator available: {}", e),
		Ok(r) => r
	};


	// let identity = PrivateIdentity::generate(&mut rng);
	let identity = {
		let private_key = PrivateKey::from_string(
			"4c80b5fee2adbd9aeb80ede1d75bd2ba93c2a6eabef38be18d4b8a418d9aa0bc")
			.expect("Invalid private key");
		
		PrivateIdentity::from_private_key(&private_key)
			.expect("Identity cannot be created from the private key")
	};

	println!("Private key: {}", identity.private_key.as_string());
	println!("Public key:  {}", identity.public_key.as_string());
	println!("Address:     {}", identity.address);
	
	let tun_interface = interface::Tun::new(&identity.address);
	println!("Opened tun device '{}'", tun_interface.get_name());

	let udp_interface = interface::Udp::create();


	let router = Router::new(&identity.address);


	let mut mio_loop: mio::EventLoop<uint, ()> = mio::EventLoop::new().unwrap();
	let event_handler = EventHandler::new(&mut mio_loop,
		tun_interface,
		udp_interface,
		router);
	
	if let Err(e) = mio_loop.run(event_handler) {
		panic!("Error while running event loop: {}", e.error);
	}
}
