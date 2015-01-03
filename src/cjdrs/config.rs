use rustc_serialize::Encodable;
use rustc_serialize::json::{self, PrettyEncoder};
use std::io::{File, IoResult};
use std::io::fs::PathExtensions;
use PrivateIdentity;

#[derive(RustcEncodable, RustcDecodable)]
#[allow(non_snake_case)]
pub struct Config {
	pub privateKey: String,
	pub tunDevice: String,
	pub udpBind: String
}

impl Config {
	pub fn get_default(identity: &PrivateIdentity) -> Config {
		Config {
			privateKey: identity.private_key.as_string(),
			tunDevice: "tun%d".to_string(),
			udpBind: "0.0.0.0:3300".to_string()
		}
	}

	pub fn write(&self, path: &Path) -> IoResult<()> {
		if path.exists() {
			panic!("Config file '{}' exists already.", path.display());
		}

		let mut file = match File::create(path) {
			Ok(file) => file,
			Err(why) => panic!("Couldn't open config file '{}': {}", path.display(), why),
		};

		let mut encoder = PrettyEncoder::new(&mut file);
		self.encode(&mut encoder)
	}

	pub fn load(path: &Path) -> Config {
		let mut file = match File::open(path) {
			Ok(file) => file,
			Err(why) => panic!("Couldn't open config file '{}': {}", path.display(), why),
		};

		let content = match file.read_to_string() {
			Ok(string) => string,
			Err(why) => panic!("Couldn't read config file '{}': {}", path.display(), why),
		};

		json::decode(content.as_slice()).unwrap()
	}
}
