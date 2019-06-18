#![feature(proc_macro_hygiene, decl_macro, custom_attribute)]

use chrono::NaiveDateTime;
use rocket::config::Environment;
use rocket::*;
use rocket_contrib::templates::tera::{self, Tera};
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::sync::Mutex;

use snippets::endpoints;
use snippets::repository::FileRepository;

/// entrypoint
///
///  * "parse args"
///  * initilize snippets repo
///  * configure and rocket and lift off
///
fn main() {
    match env::args().nth(1) {
        None => println!("parameter <repository directory> missing"),
        Some(p) => {
            // initialize our repository
            match FileRepository::new(Path::new(&p)) {
                Err(err) => println!("{}", err),
                Ok(repo) => {
                    // configure rocket and lift off
                    rocket::custom(rocket_config())
                        // mount the http endpoints
                        .mount("/", endpoints::endpoints())
                        // enable / customize the template engine
                        .attach(Template::custom(|engines| {
                            customize_tera(&mut engines.tera)
                        }))
                        // inject our repository
                        .manage(Mutex::new(repo))
                        // startup
                        .launch();
                }
            }
        }
    }
}

/// customize tera template engine
fn customize_tera(tera: &mut Tera) {
    // 'date_time' converts a unix time in a human readable date
    //   usage: {{ <unix time> | date_time }}
    tera.register_filter(
        "date_time",
        |v: tera::Value, _map: HashMap<String, tera::Value>| {
            v.as_i64()
                .ok_or(format!("invalid i64: '{}'", v).into())
                .map(|v| {
                    let date = NaiveDateTime::from_timestamp(v, 0);
                    tera::Value::String(date.format("%d.%m.%Y %H:%M:%S").to_string())
                })
        },
    )
}

/// rocket configuration
fn rocket_config() -> Config {
    Config::build(Environment::Development)
        // listen on all addresses
        .address("0.0.0.0")
        .finalize()
        .unwrap()
}
