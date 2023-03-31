#[macro_use]
extern crate rocket;

use rocket::Request;
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        parent: "layout",
        title: "Main",
    })
}

#[get("/about")]
fn about() -> Template {
    Template::render("about", context! {
        parent: "layout",
        title: "About Me",
    })
}

#[get("/legals")]
fn legals() -> Template {
    Template::render("legals", context! {
        parent: "layout",
        title: "Legals",
    })
}

#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "not_found",
        context! {
            parent: "layout",
            title: "404 - Not Found",
            url: req.uri().to_string(),
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![index, about, legals])
        .attach(Template::fairing())
}
