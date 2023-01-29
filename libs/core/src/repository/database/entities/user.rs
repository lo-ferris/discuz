use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
	#[sea_orm(primary_key, column_name = "user_id")]
	pub id: String,
	#[sea_orm(unique)]
	pub sub: String,
	pub role: String,
	pub name: String,
	#[sea_orm(unique)]
	pub email: String,
	#[sea_orm(nullable)]
	pub avatar_url: Option<String>,
	#[sea_orm(nullable)]
	pub notes: Option<String>,
	pub number_of_ban: u32,
	#[sea_orm(default_value = "normal")]
	pub status_id: String,
	pub created_at: DateTimeUtc,
	pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
	PostCategory,
	File,
	PostReply,
	BanUserId,
}

impl RelationTrait for Relation {
	fn def(&self) -> RelationDef {
		match self {
			Relation::PostCategory => Entity::has_many(super::def_post_category::Entity).into(),
			Relation::File => Entity::has_many(super::file::Entity).into(),
			Relation::PostReply => Entity::has_many(super::post_reply::Entity).into(),
			Relation::BanUserId => Entity::has_many(super::user_ban_history::Entity).into(),
		}
	}
}

impl Related<super::def_post_category::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::PostCategory.def()
	}
}

impl Related<super::file::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::File.def()
	}
}

impl Related<super::post_reply::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::PostReply.def()
	}
}

impl Related<super::user_ban_history::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::BanUserId.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

pub type User = Model;
