//! http endpoints
use crate::repository::{FileRepository, Repository};
use crate::snippet::{Snippet, Tags};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::*;
use rocket_contrib::templates::Template;
use serde::Serialize;
use std::sync::Mutex;

/// all available endpoints
pub fn endpoints() -> Vec<Route> {
    routes![index, create]
}

/// "type alias" for our state
///
///   the *state* are shared (concurrently) between all endpoints,
///   so it's wrapped in a `Mutex`.
pub type Repo<'a> = State<'a, Mutex<FileRepository>>;

/// document root
///
///  * form to create new snippets
///  * displays all snippets
///
#[get("/")]
pub fn index(repo: Repo) -> Template {
    let repo = repo.lock().unwrap();
    let snippets = repo.filter("hallo").unwrap();

    #[derive(Serialize)]
    struct IndexCtx {
        snippets: Vec<Snippet>,
    }
    Template::render("index", &IndexCtx { snippets })
}

/// this is `application/x-www-form-urlencoded` payload.
/// rocket parses the form for use, and initializes the `CreatePayload` struct
#[derive(FromForm)]
pub struct CreatePayload {
    tags: Tags,
    content: String,
}

/// create new snippet endpoint
///   * expects a `application/x-www-form-urlencoded` payload
///   * rocket parses the payload as a `CreatePayload` (because of the paramter type `Form<CreatePayload>`)
#[post("/create", data = "<payload>")]
pub fn create(repo: Repo, payload: Form<CreatePayload>) -> Redirect {
    let repo = repo.lock().unwrap();
    match repo.create(payload.tags.clone(), &payload.content) {
        Ok(_snippet) => Redirect::to("/"),
        Err(err) => {
            println!("unable to create snippet: {}", err);
            Redirect::to("/")
        }
    }
}
