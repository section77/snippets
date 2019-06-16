#![feature(proc_macro_hygiene, decl_macro, custom_attribute)]

use chrono::NaiveDateTime;
use rocket::*;
use rocket_contrib::templates::tera::{self, Tera};
use std::collections::HashMap;
use std::path::Path;
use rocket_contrib::templates::Template;
use std::sync::Mutex;

use snippets::*;

fn main() {
    // initialize our repository
    let repo = FileRepository::new(Path::new("snippets"));

    rocket::ignite()
        // mount the endpoints
        .mount("/", routes![index, create])
        // enable / customize template engine
        .attach(Template::custom(|engines| customize_tera(&mut engines.tera)))
        // inject our repository
        .manage(Mutex::new(repo))
        // startup
        .launch();
}



fn customize_tera(tera: &mut Tera) {

    // 'date_time' converts a unix time in a human readable date
    //   example: {{ <unix time> | date_time }}
    tera.register_filter(
        "date_time",
        |v: tera::Value, _map: HashMap<String, tera::Value>| {
            let date = NaiveDateTime::from_timestamp(
                v.as_i64().expect(&format!("'{}' was not a valid i64", v)),
                0,
            );
            Ok(tera::Value::String(
                date.format("%d.%m.%Y %H:%M:%S").to_string(),
            ))
        },
    )
}
