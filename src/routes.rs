use crate::templates::{ErrorTemplate, HelloTemplate, WordTemplate};
use actix_web::{
    get, post,
    web::{self, Form},
    Either, HttpResponse, Responder,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Search {
    word: String,
}

// Redirecting magic. So that URL's can be bookmarked
#[post("/search")]
pub async fn search(word: Form<Search>) -> impl Responder {
    redirect(format!("/search/{}", word.word))
}

#[get("/search/{word}")]
pub async fn search_get(word: web::Path<String>) -> Either<impl Responder, impl Responder> {
    // Redirect to error. Use Either<> to allow multiple types ([explanation](https://docs.rs/actix-web/latest/actix_web/enum.Either.html))
    thesaurus_web_lib::lookup_word(word).map_or(Either::Right(ErrorTemplate {}), |template| {
        Either::Left(WordTemplate::from(template))
    })
}

#[get("/")]
pub async fn root() -> impl Responder {
    // Root template. No render shenanigans because of askama_actix library. It allows askama templates to implement Responder
    HelloTemplate
}

#[get("/search/")]
pub async fn blank() -> impl Responder {
    // Random Get? redirect to home
    redirect("/")
}
// Redirect helper
pub fn redirect(url: impl AsRef<str>) -> HttpResponse {
    HttpResponse::SeeOther()
        .append_header(("Location", url.as_ref()))
        .finish()
}
