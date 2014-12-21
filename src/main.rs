extern crate cjdrs;
extern crate tuntap;
extern crate mio;

#[cfg(not(test))] use cjdrs::Identity;
#[cfg(not(test))] use cjdrs::TunInterface;
#[cfg(not(test))] use cjdrs::Router;
#[cfg(not(test))] use cjdrs::EventHandler;

#[cfg(not(test))]
fn main() {
	println!("Hello, cjdns!");

	let identity = Identity::generate();
	println!("Generated identity: {}", identity.ip);

	
	let tun_interface = TunInterface::new(&identity.ip);
	println!("Opened tun device '{}'", tun_interface.get_name());

	let router = Router::new(&identity.ip);


	let mut mio_loop: mio::EventLoop<uint, ()> = mio::EventLoop::new().unwrap();
	let event_handler = EventHandler::new(&mut mio_loop,
		tun_interface,
		router);
	
	if let Err(e) = mio_loop.run(event_handler) {
		panic!("Error while running event loop: {}", e.error);
	}
}
