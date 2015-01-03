use mio;
use crypto::{PasswordHash, SharedSecret};
use debug::as_hex;
use interface::NetInterface;
use packet;
use PrivateIdentity;
use Router;


pub enum Task<'a> {
	HandleIncomingPacket(packet::CryptoAuth<'a>),
	HandleOutgoingPacket(packet::IPv6<'a>)
}


pub trait EventReceiver {
	fn register(&self, event_loop: &mut mio::EventLoop<uint, ()>, token: mio::Token)
	           -> mio::MioResult<()>;
	fn receive<'a>(&'a mut self, buffer: &'a mut [u8]) -> Option<Task>;
}


pub struct EventHandler<'a> {
	my_identity: PrivateIdentity,
	interfaces: Vec<Box<NetInterface + 'a>>,
	router: Router
}

impl<'a> EventHandler<'a> {
	pub fn new(my_identity: PrivateIdentity,
	           interfaces: Vec<Box<NetInterface + 'a>>,
	           router: Router) -> EventHandler<'a> {

		EventHandler {
			my_identity: my_identity,
			interfaces: interfaces,
			router: router
		}
	}

	pub fn register_handlers(&self, event_loop: &mut mio::EventLoop<uint, ()>)
	                         -> mio::MioResult<()> {
		for (i, interface) in self.interfaces.iter().enumerate() {
			try!(interface.register(event_loop, mio::Token(i)));
		}
		Ok(())
	}
}

impl<'a> mio::Handler<uint, ()> for EventHandler<'a> {
	fn readable(&mut self, _event_loop: &mut mio::EventLoop<uint, ()>,
	            token: mio::Token, _hint: mio::event::ReadHint) {

		let mut buffer = [0u8; 1500];

		let interface_idx = token.as_uint();
		let maybe_task = self.interfaces[interface_idx].receive(&mut buffer);

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
