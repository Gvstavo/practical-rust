use actix_files::Files;
use actix_web::{http, web, App ,  HttpServer, Responder, HttpResponse, Error};
use serde::{Serialize ,Deserialize} ;
use mongodb::{Client, Database, options::FindOptions};
use futures::stream::{self, StreamExt};
use bson::oid::ObjectId;
use bson::doc;

mod models;
use self::models::*;

#[derive(Deserialize)]
struct CatEndpointPath {
    id: String,
}

fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/cats", web::get().to(cats_endpoint))
            .route("/cat/{id}", web::get().to(cat_endpoint))
    );
}

async fn cat_endpoint(pool: web::Data<Database>, cat_id: web::Path<CatEndpointPath>) -> Result<HttpResponse, Error>{
	let connection = pool;

	let id = ObjectId::with_string(&cat_id.id).unwrap(); 

	let filter = doc! {"_id": id};

	let cat_data = connection.collection("cats").find_one(filter , None).await.unwrap();

	let cat : Cat = bson::de::from_document(cat_data.unwrap()).unwrap();

	println!("{:?}", cat );

	Ok(HttpResponse::Ok().json(cat))	
}

async fn cats_endpoint(pool: web::Data<Database>) -> Result<HttpResponse, Error> {
  let connection = pool;

  let options = FindOptions::builder().limit(100).build();

  let cats_data = connection.collection("cats")
  											.find(None, options)
  											.await
  											.unwrap()
  											.filter_map(|x| async move {
  												if x.is_ok(){
  													Some(bson::de::from_document(x.unwrap()).unwrap())
  												} 
  												else {
  													None
  												}
  											})
  											.collect::<Vec<Cat>>()
  											.await;

 Ok(HttpResponse::Ok().json(cats_data))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
	let pool =  Client::with_uri_str("mongodb://localhost:27017/").await.unwrap().database("catdex");
	println!("Listening on port 8080");
	HttpServer::new(move ||{
		App::new()
		.data(pool.clone())
		.configure(api_config)
		.service(Files::new("/","static").show_files_listing())
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}