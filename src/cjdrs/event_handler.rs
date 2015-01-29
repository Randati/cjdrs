use std::time::duration::Duration;
use mio;
use crypto::{PasswordHash, SharedSecret};
use debug::as_hex;
use device::NetDevice;
use packet;
use PrivateIdentity;
use Router;


#[derive(Debug)]
pub enum Task<'a> {
	HandleIncomingPacket(packet::CryptoAuth<'a>),
	HandleOutgoingPacket(packet::IPv6<'a>)
}


pub trait EventReceiver {
	fn register(&self, event_loop: &mut mio::EventLoop<usize, ()>, token: mio::Token)
	            -> mio::MioResult<()>;
	fn receive<'a>(&'a mut self, buffer: &'a mut [u8]) -> Option<Task>;
}


#[derive(Debug)]
pub struct EventHandler<'a> {
	my_identity: PrivateIdentity,
	devices: Vec<Box<NetDevice + 'a>>,
	router: Router
}

impl<'a> EventHandler<'a> {
	pub fn new(my_identity: PrivateIdentity,
	           devices: Vec<Box<NetDevice + 'a>>,
	           router: Router) -> EventHandler<'a> {

		EventHandler {
			my_identity: my_identity,
			devices: devices,
			router: router
		}
	}

	pub fn register_handlers(&self, event_loop: &mut mio::EventLoop<usize, ()>)
	                         -> mio::MioResult<()> {
		for (i, device) in self.devices.iter().enumerate() {
			try!(device.register(event_loop, mio::Token(i)));
		}
		event_loop.timeout(1000, Duration::milliseconds(0)).unwrap();
		Ok(())
	}
}

impl<'a> mio::Handler<usize, ()> for EventHandler<'a> {
	fn timeout(&mut self, event_loop: &mut mio::EventLoop<usize, ()>, timeout: usize) {
		assert_eq!(timeout, 1000);
	
		event_loop.timeout(1000, Duration::milliseconds(1000)).unwrap();
	}
	
	fn readable(&mut self, _event_loop: &mut mio::EventLoop<usize, ()>,
	            token: mio::Token, _hint: mio::event::ReadHint) {

		let mut buffer = [0u8; 1500];

		let device_idx = token.as_usize();
		let maybe_task = self.devices[device_idx].receive(&mut buffer);

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
					let destination = ipv6_packet.get_destination().unwrap();
					println!("Handling outgoing packet to {}", destination);
					
					let route = self.router.get_route(&destination);
					println!("    Route: {}", route);
				}
			}
		}
	}
}
