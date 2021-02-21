use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

#[derive(Serialize,Deserialize, Debug)]
pub struct Cat {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub name: String,
	pub image_path: String

}