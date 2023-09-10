#[marco_use] extern crate rocket;
#![feature(decl_macro)]

use rocket::repsonse::content::Json;

#[get("/hello")]
fn hello() -> Json<&'static str> {
	Json("{
		'status': 'success',
		'message': 'Hello API!'
	}")
}

fn main() {
	rocket::ignite()
		.mount("/api", routes![hello])
		.launch();
}


