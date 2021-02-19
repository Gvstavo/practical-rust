use actix_files::Files;
use actix_web::{http, web, App ,  HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct Cat{
	pub id: i32,
	pub name: String,
	pub image_path: String
}

async fn cats() -> impl Responder{

	let cats = vec![
		Cat{
			id: 1,
			name: "foo".to_string(),
			image_path: "foo.png".to_string()
		},
		Cat{
			id: 2,
			name: "bar".to_string(),
			image_path: "bar.png".to_string()
		},
	];

	return web::Json(cats);
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
	println!("Listening on port 8080");
	HttpServer::new(move ||{
		App::new()
		.service(web::scope("/api").route("/cats", web::get().to(cats)))
		.service(Files::new("/","static").show_files_listing())
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}