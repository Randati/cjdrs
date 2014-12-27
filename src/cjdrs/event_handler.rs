extern crate mio;

use Router;
use interface;

const TUN_INCOMING: mio::Token = mio::Token(1);
const UDP_INCOMING: mio::Token = mio::Token(2);


pub trait EventReceiver {
	fn register(&self, event_loop: &mut mio::EventLoop<uint, ()>, token: mio::Token)
	           -> mio::MioResult<()>;
	fn receive(&mut self);
}

pub struct EventHandler {
	tun_interface: interface::Tun,
	udp_interface: interface::Udp,
	router: Router
}

impl EventHandler {
	pub fn new(event_loop: &mut mio::EventLoop<uint, ()>,
	           tun_interface: interface::Tun,
	           udp_interface: interface::Udp,
	           router: Router) -> EventHandler {
		
		let event_handler = EventHandler {
			tun_interface: tun_interface,
			udp_interface: udp_interface,
			router: router
		};

		if let Err(e) = event_handler.register_handlers(event_loop) {
			panic!("Couldn't register event handlers: {}", e);
		}

		event_handler
	}

	pub fn register_handlers(&self, event_loop: &mut mio::EventLoop<uint, ()>)
	                        -> mio::MioResult<()> {
		try!(self.tun_interface.register(event_loop, TUN_INCOMING));
		try!(self.udp_interface.register(event_loop, UDP_INCOMING));
		Ok(())
	}
}

impl mio::Handler<uint, ()> for EventHandler {
	fn readable(&mut self, _event_loop: &mut mio::EventLoop<uint, ()>,
	            token: mio::Token, _hint: mio::event::ReadHint) {

		match token {
			TUN_INCOMING => self.tun_interface.receive(),
			UDP_INCOMING => self.udp_interface.receive(),
			_ => panic!("Unknown event type {}", token)
		};
	}
}
