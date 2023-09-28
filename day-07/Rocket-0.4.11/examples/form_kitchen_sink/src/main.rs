#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::request::{Form, FormError, FormDataError};
use rocket::response::NamedFile;
use rocket::http::RawStr;

#[cfg(test)] mod tests;

#[derive(Debug, FromFormValue)]
enum FormOption {
    A, B, C
}

#[derive(Debug, FromForm)]
struct FormInput<'r> {
    checkbox: bool,
    number: usize,
    #[form(field = "type")]
    radio: FormOption,
    password: &'r RawStr,
    #[form(field = "textarea")]
    text_area: String,
    select: FormOption,
}

#[post("/", data = "<sink>")]
fn sink(sink: Result<Form<FormInput>, FormError>) -> String {
    match sink {
        Ok(form) => format!("{:?}", &*form),
        Err(FormDataError::Io(_)) => format!("Form input was invalid UTF-8."),
        Err(FormDataError::Malformed(f)) | Err(FormDataError::Parse(_, f)) => {
            format!("Invalid form input: {}", f)
        }
    }
}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, sink])
}

fn main() {
    rocket().launch();
}
