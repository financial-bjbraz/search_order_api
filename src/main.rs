#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

use rocket_contrib::json::{Json, JsonValue};

mod user;
use user::{User};
use user::{UserDTO};

mod db;
mod schema;
use self::schema::users;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/", data = "<user>")]
fn create(user: Json<UserDTO>, connection: db::Connection) -> Json<UserDTO> {
    let insert = UserDTO { ..user.into_inner() };
    Json(User::create(insert, &connection))
}

#[put("/<id>", data = "<user>")]
fn update(id: i32, user: Json<User>, connection: db::Connection) -> JsonValue {
    let update = User { id: (id), ..user.into_inner() };
    json!({
        "success": User::update(id, update, &connection)
    })
}

#[get("/<id>")]
fn get(id: i32, connection: db::Connection) -> JsonValue {
    json!({
        "success": User::select(id, &connection)
    })
}

#[delete("/<id>")]
fn delete(id: i32, connection: db::Connection) -> JsonValue {
    json!({
        "success": User::delete(id, &connection)
    })
}

#[get("/")]
fn read(connection: db::Connection) -> JsonValue {
    json!(User::read(&connection))
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index])
    .mount("/user", routes![create, update,  delete, get])
    .mount("/users", routes![read]) 
    .manage(db::connect())
    .launch();
}