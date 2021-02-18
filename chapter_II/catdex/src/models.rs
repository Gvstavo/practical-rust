use serde::{Deserialize, Serialize};
use super::schema::cats;

#[derive(Queryable, Serialize)]
pub struct Cat{
	pub id: i32,
	pub name: String,
	pub image_path: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "cats"]
pub struct NewCat{
	pub name: String,
	pub image_path: String
}