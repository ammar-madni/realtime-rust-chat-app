#[macro_use] extern crate rocket;

#[get("/hello-world")]
fn hello_world() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello_world])
}
