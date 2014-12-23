extern crate cjdrs;
extern crate mio;

#[cfg(not(test))] use std::rand::OsRng;
#[cfg(not(test))] use cjdrs::PrivateIdentity;
#[cfg(not(test))] use cjdrs::TunInterface;
#[cfg(not(test))] use cjdrs::Router;
#[cfg(not(test))] use cjdrs::EventHandler;

#[cfg(not(test))]
fn main() {
	println!("Hello, cjdns!");

	let mut rng = match OsRng::new() {
		Err(e) => panic!("No random number generator available: {}", e),
		Ok(r) => r
	};


	let identity = PrivateIdentity::generate(&mut rng);
	println!("Generated identity: {}", identity.address);

	
	let tun_interface = TunInterface::new(&identity.address);
	println!("Opened tun device '{}'", tun_interface.get_name());

	let router = Router::new(&identity.address);


	let mut mio_loop: mio::EventLoop<uint, ()> = mio::EventLoop::new().unwrap();
	let event_handler = EventHandler::new(&mut mio_loop,
		tun_interface,
		router);
	
	if let Err(e) = mio_loop.run(event_handler) {
		panic!("Error while running event loop: {}", e.error);
	}
}
