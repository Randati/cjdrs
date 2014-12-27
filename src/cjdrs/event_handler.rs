extern crate mio;

use Router;
use TunInterface;

const TUN_INCOMING: mio::Token = mio::Token(1);


pub struct EventHandler {
	tun_interface: TunInterface,
	_router: Router
}

impl EventHandler {
	pub fn new(event_loop: &mut mio::EventLoop<uint, ()>,
	           tun_interface: TunInterface,
	           router: Router) -> EventHandler {
		
		let event_handler = EventHandler {
			tun_interface: tun_interface,
			_router: router
		};

		if let Err(e) = event_handler.register_handlers(event_loop) {
			panic!("Couldn't register event handlers: {}", e);
		}

		event_handler
	}

	pub fn register_handlers(&self, event_loop: &mut mio::EventLoop<uint, ()>
	                        ) -> mio::MioResult<()> {
		try!(event_loop.register(&self.tun_interface, TUN_INCOMING));
		Ok(())
	}
}

impl mio::Handler<uint, ()> for EventHandler {
	fn readable(&mut self, _event_loop: &mut mio::EventLoop<uint, ()>,
	            token: mio::Token, _hint: mio::event::ReadHint) {

		let mut buffer = [0u8, ..1500];

		match token {
			TUN_INCOMING => {
				match self.tun_interface.read_incoming_packet(&mut buffer) {
					Ok(tun_packet) => {
						let ipv6_packet = &tun_packet.get_data();
						let destination = ipv6_packet.get_destination();
						println!("Tun -> {}", destination);
					},
					Err(e) => println!("Couldn't parse packet: {}", e)
				}
			},
			_ => panic!("Unknown event type {}", token)
		};
	}
}
