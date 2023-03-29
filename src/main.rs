#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::templates::Template;

#[derive(serde::Serialize)]
struct BoardContext {
    parent: &'static str,
}

#[derive(serde::Serialize)]
struct AboutContext {
    parent: &'static str,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", &BoardContext { parent: "layout" })
}

#[get("/about")]
fn about() -> Template {
    Template::render("about", &AboutContext { parent: "layout" })
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![about])
        .mount("/", routes![index])
        .launch();
}
