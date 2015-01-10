use std::{io, error, fmt};
use mio;
use rustc_serialize::{hex, json};
use identity::PRIV_KEY_SIZE;
use PrivateKey;
use PublicKey;

use CjdrsError::{
	ConfigAlreadyExists,
	InvalidPrivateKey,
	InvalidPublicKey,
	NoAddressForPrivateKey,
	NoAddressForPublicKey,
	InvalidBindAddress,
	JsonDecodingError,
	MioError,
	FmtError,
	IoError,
};

pub type CjdrsResult<T> = Result<T, CjdrsError>;

pub enum CjdrsError {
	ConfigAlreadyExists(Path),
	InvalidPrivateKey(Option<hex::FromHexError>),
	InvalidPublicKey,
	NoAddressForPrivateKey(PrivateKey),
	NoAddressForPublicKey(PublicKey),
	InvalidBindAddress(String),
	JsonDecodingError(json::DecoderError),
	MioError(mio::MioError),
	FmtError(fmt::Error),
	IoError(io::IoError),
}


impl error::Error for CjdrsError {
	fn description(&self) -> &str {
		match *self {
			ConfigAlreadyExists(..) => "Configuration file aready exists",
			InvalidPrivateKey(..) => "Invalid private key",
			InvalidPublicKey => "Invalid public key",
			NoAddressForPrivateKey(..) => "Private key has no valid IP address",
			NoAddressForPublicKey(..) => "Public key has no valid IP address",
			InvalidBindAddress(..) => "Invalid bind address",
			JsonDecodingError(..) => "JSON decoding error",
			MioError(..) => "Event handler error",
			FmtError(..) => "Formatting error",
			IoError(..) => "I/O error",
		}
	}

	fn detail(&self) -> Option<String> {
		match *self {
			ConfigAlreadyExists(ref path) =>
				Some(format!("Path '{}'", path.display())),

			InvalidPrivateKey(Some(ref e)) =>
				Some(format!("{:?}", e)),

			InvalidPrivateKey(None) =>
				Some(format!("Private key must be {} characters long", PRIV_KEY_SIZE * 2)),
			
			InvalidPublicKey =>
				Some("Public key must be 54 character base32 encoded string including '.k'".to_string()),
			
			NoAddressForPrivateKey(ref k) =>
				Some(format!("Private key '{}'", k)),

			NoAddressForPublicKey(ref k) =>
				Some(format!("Public key '{}'", k)),

			InvalidBindAddress(ref s) =>
				Some(format!("Bind address '{}' is invalid", s)),
			
			JsonDecodingError(ref e) =>
				Some(format!("{:?}", e)),
			
			MioError(ref e) =>
				Some(format!("{:?}", e)),
			
			FmtError(ref e) =>
				Some(format!("{:?}", e)),

			IoError(ref e) =>
				Some(format!("{:?}", e)),
		}
	}

	fn cause(&self) -> Option<&error::Error> {
		match *self {
			JsonDecodingError(ref e) => Some(e as &error::Error),
			IoError(ref e) => Some(e as &error::Error),
			_ => None,
		}
	}
}


impl fmt::Show for CjdrsError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match error::Error::detail(self) {
			Some(detail) => write!(f, "{}: {}", error::Error::description(self), detail),
			None         => write!(f, "{}", error::Error::description(self)),
		}
	}
}


impl error::FromError<io::IoError> for CjdrsError {
	fn from_error(e: io::IoError) -> CjdrsError {
		CjdrsError::IoError(e)
	}
}

impl error::FromError<fmt::Error> for CjdrsError {
	fn from_error(e: fmt::Error) -> CjdrsError {
		CjdrsError::FmtError(e)
	}
}

impl<H> error::FromError<mio::EventLoopError<H>> for CjdrsError {
	fn from_error(e: mio::EventLoopError<H>) -> CjdrsError {
		CjdrsError::MioError(e.error)
	}
}

impl error::FromError<mio::MioError> for CjdrsError {
	fn from_error(e: mio::MioError) -> CjdrsError {
		CjdrsError::MioError(e)
	}
}

impl error::FromError<json::DecoderError> for CjdrsError {
	fn from_error(e: json::DecoderError) -> CjdrsError {
		CjdrsError::JsonDecodingError(e)
	}
}
