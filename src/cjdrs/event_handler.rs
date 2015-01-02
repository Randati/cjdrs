use mio;
use crypto::{PasswordHash, SharedSecret};
use debug::as_hex;
use interface;
use packet;
use PrivateIdentity;
use Router;

const TUN_INCOMING: mio::Token = mio::Token(1);
const UDP_INCOMING: mio::Token = mio::Token(2);


pub enum Task<'a> {
	HandleIncomingPacket(packet::CryptoAuth<'a>),
	HandleOutgoingPacket(packet::IPv6<'a>)
}


pub trait EventReceiver {
	fn register(&self, event_loop: &mut mio::EventLoop<uint, ()>, token: mio::Token)
	           -> mio::MioResult<()>;
	fn receive<'a>(&'a mut self, buffer: &'a mut [u8]) -> Option<Task>;
}


pub struct EventHandler {
	my_identity: PrivateIdentity,
	tun_interface: interface::Tun,
	udp_interface: interface::Udp,
	router: Router
}

impl EventHandler {
	pub fn new(event_loop: &mut mio::EventLoop<uint, ()>,
	           my_identity: PrivateIdentity,
	           tun_interface: interface::Tun,
	           udp_interface: interface::Udp,
	           router: Router) -> mio::MioResult<EventHandler> {
		
		let event_handler = EventHandler {
			my_identity: my_identity,
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
				Task::HandleIncomingPacket(mut ca_packet) => {
					println!("Handling incoming packet");

					let password_hash = PasswordHash::from_password("aaa");
					
					let shared_secret = SharedSecret::with_password(
						&self.my_identity.private_key,
						&ca_packet.public_key(),
						&password_hash);
					
					match ca_packet.decrypt(&shared_secret) {
						Some(message) => println!("Decrypted message: {}", as_hex(message.as_slice())),
						None => println!("Couldn't decrypt the message!")
					}

				},
				Task::HandleOutgoingPacket(ipv6_packet) => {
					let destination = ipv6_packet.get_destination();
					println!("Handling outgoing packet to {}", destination);
					
					let route = self.router.get_route(&destination);
					println!("    Route: {}", route);
				}
			}
		}
	}
}
