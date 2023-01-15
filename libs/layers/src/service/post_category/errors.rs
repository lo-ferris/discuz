use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum PostCategoryError {
	#[display(fmt = "Post Category Error: Generic {}", _0)]
	Generic(#[error(not(source))] String),
}
