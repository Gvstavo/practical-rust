use actix_files::Files;
use actix_web::{http, web, App ,  HttpServer, Responder, HttpResponse, Error};
use serde::{Serialize ,Deserialize} ;
use mongodb::{Client, Database, options::FindOptions};
use futures::stream::{self, StreamExt};
use bson::oid::ObjectId;
use bson::doc;
use actix_web::middleware::Logger;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod models;
mod errors;
use self::errors::UserError;
use self::models::*;

#[derive(Deserialize)]
struct CatEndpointPath {
    id: String,
}

#[derive(Debug,Serialize, Deserialize)]
struct FormData {
    name: String,
    image: String
}


fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
           .route("/cats", web::get().to(cats_endpoint))
           .route("/cat/{id}", web::get().to(cat_endpoint))
           .route("/cat", web::post().to(cat_endpoint_new))
    );
}

async fn cat_endpoint_new(pool: web::Data<Database>, form: web::Form<FormData>) ->  Result<HttpResponse, UserError>{

  println!("entrando no post");

  let connection = pool;

  println!("{:?}",form);

  Ok(HttpResponse::Ok().finish())

}

async fn cat_endpoint(pool: web::Data<Database>, cat_id: web::Path<CatEndpointPath>) -> Result<HttpResponse, UserError>{
	let connection = pool;

	let id = ObjectId::with_string(&cat_id.id).map_err(|_| UserError::ValidationError)?;

	let filter = doc! {"_id": id};

	let cat_data = connection.collection("cats").find_one(filter , None).await.map_err(|_| UserError::UnexpectedError)?;

	let cat : Cat = bson::de::from_document(cat_data.ok_or_else(|| UserError::NotFoundError)?).map_err(|_| UserError::UnexpectedError)?;

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
	env_logger::init();
	let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
   
  builder.set_private_key_file("key-no-password.pem",SslFiletype::PEM,).unwrap();
	
	builder.set_certificate_chain_file("cert.pem").unwrap();


	let pool =  Client::with_uri_str("mongodb://localhost:27017/").await.unwrap().database("catdex");
	println!("Listening on port 8080");
	HttpServer::new(move ||{
		App::new()
		.data(pool.clone())
		.wrap(Logger::default())
		.configure(api_config)
		.service(Files::new("/","static").index_file("index.html"))
	})
	.bind_openssl("127.0.0.1:8080", builder)?
	.run()
	.await
}