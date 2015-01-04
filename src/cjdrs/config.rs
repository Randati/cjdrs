use rustc_serialize::Encodable;
use rustc_serialize::json::{mod, PrettyEncoder};
use std::io::File;
use std::io::fs::PathExtensions;
use crypto::random_password;
use PrivateIdentity;
use CjdrsResult;
use CjdrsError;

#[deriving(RustcEncodable, RustcDecodable)]
#[allow(non_snake_case)]
pub struct Config {
	pub privateKey: String,
	pub tunDevice: String,
	pub udpBind: String,
	pub authorizedPasswords: Vec<String>
}

impl Config {
	pub fn get_default(identity: &PrivateIdentity) -> Config {
		Config {
			privateKey: identity.private_key.as_string(),
			tunDevice: "tun%d".to_string(),
			udpBind: "0.0.0.0:3300".to_string(),
			authorizedPasswords: vec![
				random_password()
			]
		}
	}

	pub fn write(&self, path: &Path) -> CjdrsResult<()> {
		if path.exists() {
			fail!(CjdrsError::ConfigAlreadyExists(path.clone()));
		}

		let mut file = try!(File::create(path));

		let mut encoder = PrettyEncoder::new(&mut file);
		Ok(try!(self.encode(&mut encoder)))
	}

	pub fn load(path: &Path) -> CjdrsResult<Config> {
		let mut file = try!(File::open(path));
		let content = try!(file.read_to_string());
		Ok(try!(json::decode(content.as_slice())))
	}
}
