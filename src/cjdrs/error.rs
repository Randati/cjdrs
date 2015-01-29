use std::{old_io, error, fmt};
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
	JsonEncodingError,
	MioError,
	FmtError,
	IoError,
};

pub type CjdrsResult<T> = Result<T, CjdrsError>;

#[derive(Debug)]
pub enum CjdrsError {
	ConfigAlreadyExists(Path),
	InvalidPrivateKey(Option<hex::FromHexError>),
	InvalidPublicKey,
	NoAddressForPrivateKey(PrivateKey),
	NoAddressForPublicKey(PublicKey),
	InvalidBindAddress(String),
	JsonDecodingError(json::DecoderError),
	JsonEncodingError(json::EncoderError),
	MioError(mio::MioError),
	FmtError(fmt::Error),
	IoError(old_io::IoError),
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
			JsonEncodingError(..) => "JSON encoding error",
			MioError(..) => "Event handler error",
			FmtError(..) => "Formatting error",
			IoError(..) => "I/O error",
		}
	}

	fn cause(&self) -> Option<&error::Error> {
		match *self {
			JsonDecodingError(ref e) => Some(e as &error::Error),
			JsonEncodingError(ref e) => Some(e as &error::Error),
			IoError(ref e) => Some(e as &error::Error),
			_ => None,
		}
	}
}

impl fmt::Display for CjdrsError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		try!(write!(f, "{}. ", error::Error::description(self)));

		match *self {
			ConfigAlreadyExists(ref path) =>
				write!(f, "Path '{}'", path.display()),

			InvalidPrivateKey(Some(ref e)) =>
				write!(f, "{:?}", e),

			InvalidPrivateKey(None) =>
				write!(f, "Private key must be {} characters long", PRIV_KEY_SIZE * 2),
			
			InvalidPublicKey =>
				write!(f, "Public key must be 54 character base32 encoded string including '.k'"),
			
			NoAddressForPrivateKey(ref k) =>
				write!(f, "Private key '{}'", k),

			NoAddressForPublicKey(ref k) =>
				write!(f, "Public key '{}'", k),

			InvalidBindAddress(ref s) =>
				write!(f, "Bind address '{}' is invalid", s),
			
			JsonDecodingError(ref e) =>
				write!(f, "{:?}", e),

			JsonEncodingError(ref e) =>
				write!(f, "{:?}", e),
			
			MioError(ref e) =>
				write!(f, "{:?}", e),
			
			FmtError(ref e) =>
				write!(f, "{:?}", e),

			IoError(ref e) =>
				write!(f, "{:?}", e),
		}
	}
}


impl error::FromError<old_io::IoError> for CjdrsError {
	fn from_error(e: old_io::IoError) -> CjdrsError {
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


impl error::FromError<json::EncoderError> for CjdrsError {
	fn from_error(e: json::EncoderError) -> CjdrsError {
		CjdrsError::JsonEncodingError(e)
	}
}
