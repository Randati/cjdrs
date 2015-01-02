use mio;
use Router;
use interface;
use packet;

const TUN_INCOMING: mio::Token = mio::Token(1);
const UDP_INCOMING: mio::Token = mio::Token(2);


pub enum Task<'a> {
	HandleIncomingPacket(packet::CryptoAuth<'a>),
	HandleOutgoingPacket(&'a [u8])
}


pub trait EventReceiver {
	fn register(&self, event_loop: &mut mio::EventLoop<uint, ()>, token: mio::Token)
	           -> mio::MioResult<()>;
	fn receive<'a>(&'a mut self, buffer: &'a mut [u8]) -> Option<Task>;
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
	           router: Router) -> mio::MioResult<EventHandler> {
		
		let event_handler = EventHandler {
			tun_interface: tun_interface,
			udp_interface: udp_interface,
			router: router
		};
		
		try!(event_handler.tun_interface.register(event_loop, TUN_INCOMING));
		try!(event_handler.udp_interface.register(event_loop, UDP_INCOMING));

		Ok(event_handler)
	}
}

impl mio::Handler<uint, ()> for EventHandler {
	fn readable(&mut self, _event_loop: &mut mio::EventLoop<uint, ()>,
	            token: mio::Token, _hint: mio::event::ReadHint) {

		let mut buffer = [0u8; 1500];

		let maybe_task = match token {
			TUN_INCOMING => self.tun_interface.receive(&mut buffer),
			UDP_INCOMING => self.udp_interface.receive(&mut buffer),
			_ => panic!("Unknown event type {}", token)
		};

		if let Some(task) = maybe_task {
			match task {
				Task::HandleIncomingPacket(packet) => {
					println!("Handling incoming packet");
				},
				Task::HandleOutgoingPacket(data) => {
					println!("Handling outgoing packet");
				}
			}
		}
	}
}
