
pub trait NetInterface {
	fn send_message(&self, msg: &str);
	fn receive_message(&self) -> String;
}
