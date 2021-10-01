use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sailfish::TemplateOnce;
use serde::Deserialize;

mod database;
use database::*;
use std::sync::Mutex;

#[derive(TemplateOnce)]
#[template(path = "home.stpl")]
struct Home {}

#[derive(TemplateOnce)]
#[template(path = "list.stpl")]
struct List {
    persons: Vec<String>,
}

#[derive(Deserialize)]
struct Person {
    name: String,
}

async fn homepage() -> impl Responder {
    HttpResponse::Ok().body(Home {}.render_once().unwrap())
}

async fn add_to_persons(
    person: web::Form<Person>,
    db_mutex: web::Data<Mutex<Db>>,
) -> impl Responder {
    let mut db = db_mutex.lock().unwrap();
    db.add(person.name.clone());
    println!("{:?}", db.persons);
    HttpResponse::Found().header("Location", "/").finish()
}

async fn get_persons(db_mutex: web::Data<Mutex<Db>>) -> impl Responder {
    let db = db_mutex.lock().unwrap();
    let persons_list = db.get();
    HttpResponse::Ok().body(
        List {
            persons: persons_list,
        }
        .render_once()
        .unwrap(),
    )
}

async fn delete_person(
    db_mutex: web::Data<Mutex<Db>>,
    web::Path((id,)): web::Path<(usize,)>,
) -> impl Responder {
    let mut db = db_mutex.lock().unwrap();
    db.delete(id);
    HttpResponse::Found().header("Location", "/list").finish()
}

#[actix_web::main]
async fn main() {
    let addr = "localhost:8080";

    let db = web::Data::new(Mutex::new(Db::new()));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .route("/", web::get().to(homepage))
            .route("/add", web::post().to(add_to_persons))
            .route("/list", web::get().to(get_persons))
            .service(web::resource("/delete/{id}").route(web::get().to(delete_person)))
    })
    .bind(addr)
    .unwrap()
    .run();

    println!("Server live at http://{}", addr);
    server.await.unwrap();
}
