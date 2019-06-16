use crate::repository::Repository;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::*;
use rocket_contrib::templates::Template;
use serde::Serialize;
use std::sync::Mutex;

// "type alias" für unseren state
type Repo<'a> = State<'a, Mutex<crate::FileRepository>>;

#[get("/")]
pub fn index(repo: Repo) -> Template {
    let repo = repo.lock().unwrap();
    let snippets = repo.list().unwrap();

    #[derive(Serialize)]
    struct IndexCtx {
        snippets: Vec<crate::Snippet>,
    }
    Template::render("index", &IndexCtx { snippets })
}

#[derive(FromForm)]
pub struct CreatePayload {
    tags: crate::Tags,
    content: String,
}

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
