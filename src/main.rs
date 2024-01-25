#[macro_use] extern crate rocket;
extern crate rocket_dyn_templates;
extern crate comrak;
extern crate tera;

use std::path::{ PathBuf, Path };
use rocket::fs::NamedFile;
use rocket::response::content;
use comrak::{markdown_to_html_with_plugins, Options, Plugins};
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
  Template::render("base", context!{
    content: r#"<a href="/blogs/example.md">link</a>"#
  })
}

#[get("/styles/<file>")]
async fn styles(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("styles/").join(file)).await.ok()
}

#[get("/blogs/<file>")]
async fn blogs(file: String) -> Template {
  let content = std::fs::read_to_string(format!("./blogs/{}", file));
  let mut option = Options::default();
  option.extension.autolink = true;
  option.extension.tasklist = true;
  let text = match content {
    Ok(text) => markdown_to_html_with_plugins(text.as_str(), &option, &Plugins::default()),
    Err(_err) => String::new(),
  };
  Template::render("base", context!{
    content: text,
  })
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount("/", routes![index, styles, blogs])
    .attach(Template::fairing())
}
