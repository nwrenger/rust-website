#[macro_use]
extern crate rocket;

use rocket::Request;
use rocket_dyn_templates::{Template, context};

use std::path::{PathBuf, Path};
use rocket::fs::NamedFile;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        parent: "layout",
        title: "Home",
    })
}

#[get("/<path..>")]
pub async fn static_files(path: PathBuf) -> Option<NamedFile> {
    let path = Path::new("static").join(path);
    NamedFile::open(path).await.ok()
}

#[get("/projects")]
fn projects() -> Template {
    Template::render("projects", context! {
        parent: "layout",
        title: "My Projects",
    })
}

#[get("/about")]
fn about() -> Template {
    Template::render("about", context! {
        parent: "layout",
        title: "About Me",
    })
}

#[get("/contact")]
fn contact() -> Template {
    Template::render("contact", context! {
        parent: "layout",
        title: "Contact",
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
        .mount("/", routes![index, projects, about, contact, legals, static_files])
        .attach(Template::fairing())
}
