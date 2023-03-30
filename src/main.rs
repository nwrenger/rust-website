#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::templates::Template;
use rocket::Request;

#[derive(serde::Serialize)]
struct BoardContext {
    parent: &'static str,
}

#[derive(serde::Serialize)]
struct AboutContext {
    parent: &'static str,
}

#[derive(serde::Serialize)]
struct LegalsContext {
    parent: &'static str,
}

#[derive(serde::Serialize)]
struct NotFoundContext {
    parent: &'static str,
    url: String,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", &BoardContext { parent: "layout" })
}

#[get("/about")]
fn about() -> Template {
    Template::render("about", &AboutContext { parent: "layout" })
}

#[get("/legals")]
fn legals() -> Template {
    Template::render("legals", &AboutContext { parent: "layout" })
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    Template::render("not_found", &NotFoundContext { 
        parent: "layout",
        url: req.uri().to_string(),
    })
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .register(catchers![not_found])
        .mount("/", routes![index, about, legals])
        .launch();
}
