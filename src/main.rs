#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate lazy_static;
extern crate diesel;
extern crate mlib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate r2d2;
extern crate r2d2_diesel;

// Used for Routes
use rocket::Request;
use rocket::response::NamedFile;
use rocket_contrib::Template;

// Server imports
// Used to Setup DB Pool
use rocket::request::{Outcome, FromRequest};
use rocket::Outcome::{Success, Failure};
use rocket::http::Status;

// DB imports
use diesel::prelude::*;
use diesel::update;
use diesel::pg::PgConnection;
use r2d2::{Pool, PooledConnection, GetTimeout};
use r2d2_diesel::ConnectionManager;
use mlib::models::*;
use mlib::*;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

lazy_static! {
    pub static ref DB_POOL: Pool<ConnectionManager<PgConnection>> = create_db_pool();
}

pub struct DB(PooledConnection<ConnectionManager<PgConnection>>);

impl DB {
    pub fn conn(&self) -> &PgConnection {
        &*self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = GetTimeout;
    fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match DB_POOL.get() {
            Ok(conn) => Success(DB(conn)),
            Err(e) => Failure((Status::InternalServerError, e)),
        }
    }
}

#[get("/assets/<path..>")]
fn render_asset(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(path)).ok()
}

#[get("/")]
fn index(db: DB) -> Template {
    use mlib::schema::users::dsl::*;
    let query = users.first::<User>(db.conn()).expect("Error loading users");
    //let serialized = serde_json::to_value(&query);
    //println!("query = {:?}", &serialized);
    //let mut data = HashMap::new();
    //data.insert("users", "sato");
    Template::render("index", &query)
    //Template::render("index", &serialized)
}

#[error(404)]
fn not_found(req: &Request) -> String {
    format!("not found {}", req)
}

#[get("/welcome/<username>")]
fn welcome(username: &str) -> String {
    format!("Welcome, {}!", username)
}

fn main() {
    rocket::ignite().mount("/", routes![render_asset, index, welcome]).catch(errors![not_found]).launch();
}
