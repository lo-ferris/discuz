use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum AuthError {
	#[display(fmt = "Auth Error: Generic {}", _0)]
	Generic(#[error(not(source))] String),
	#[display(fmt = "Auth Error: Invalid access token")]
	InvalidAccessTokenError,
}
