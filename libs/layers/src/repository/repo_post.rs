use derive_more::{Display, Error};
use error_stack::{IntoReport, Result, ResultExt};

pub use crate::repository::database::db_post::{
	CreatePostInput, CreatePostTagInput, DbPost, DbPostTrait, DefPostTag, Post, PostTag,
	UploadPostInput, UploadPostTagInput,
};

#[derive(Debug, Clone)]
pub struct RepoPost {
	db_post: DbPost,
}

impl RepoPost {
	pub fn new(db_post: DbPost) -> RepoPost {
		RepoPost { db_post }
	}
}

#[async_trait]
pub trait RepoPostTrait {
	// Post
	async fn find_by_id(&self, id: &str) -> Result<Option<Post>, RepoError>;
	async fn create(&self, input: &CreatePostInput) -> Result<String, RepoError>;
	async fn update(&self, input: &UploadPostInput) -> Result<(), RepoError>;
	// Def Post Tag
	async fn find_post_tag_by_id(&self, id: &str) -> Result<Option<DefPostTag>, RepoError>;
	async fn create_post_tag(&self, input: &CreatePostTagInput) -> Result<String, RepoError>;
	async fn update_post_tag(&self, input: &UploadPostTagInput) -> Result<(), RepoError>;
	//
	async fn find_post_tags_by_post_id(&self, id: &str) -> Result<Vec<PostTag>, RepoError>;
}

#[async_trait]
impl RepoPostTrait for RepoPost {
	// Post
	async fn find_by_id(&self, id: &str) -> Result<Option<Post>, RepoError> {
		self.db_post
			.find_by_id(id)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn create(&self, input: &CreatePostInput) -> Result<String, RepoError> {
		self.db_post
			.create(input)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn update(&self, input: &UploadPostInput) -> Result<(), RepoError> {
		self.db_post
			.update(input)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	// Def Post Tag
	async fn find_post_tag_by_id(&self, id: &str) -> Result<Option<DefPostTag>, RepoError> {
		self.db_post
			.find_post_tag_by_id(id)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn create_post_tag(&self, input: &CreatePostTagInput) -> Result<String, RepoError> {
		self.db_post
			.create_post_tag(input)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn update_post_tag(&self, input: &UploadPostTagInput) -> Result<(), RepoError> {
		self.db_post
			.update_post_tag(input)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn find_post_tags_by_post_id(&self, id: &str) -> Result<Vec<PostTag>, RepoError> {
		self.db_post
			.find_post_tags_by_post_id(id)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}
}

#[derive(Debug, Error, Display)]
pub enum RepoError {
	#[display(fmt = "Repo Post Error: Generic")]
	Generic,
}