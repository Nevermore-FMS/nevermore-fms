use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use actix_web::{get, web::{self, Form}, App, HttpServer, HttpResponse, http::{header::ContentType, Method, StatusCode}, Responder, post, cookie::Key};
use actix_web_lab::web::Redirect;
use handlebars::Handlebars;
use jfs::Store;
use serde_derive::Deserialize;
use serde_json::json;
use std::{sync::Arc, path::Path, collections::HashMap};

use crate::{field::Field, plugin::PluginManager};

use self::users::{Users, User};

pub mod users;

fn verify_session(session: Session) -> Option<User> {
    if session.get::<User>("user").is_err() {
        return None;
    } else {
        return session.get::<User>("user").unwrap();
    }
}

#[get("/")]
async fn index(plugin_manager: web::Data<PluginManager>, hb: web::Data<Arc<Handlebars<'_>>>, session: Session) -> actix_web::Result<impl Responder> {
    if verify_session(session).is_none() {
        return Ok(HttpResponse::Found().insert_header(("Location", "/login")).finish());
    }
    let out = hb.render("index.hbl", &json!({
        "plugins": plugin_manager.get_plugins_metadata().await,
        "plugin_token": plugin_manager.get_plugin_registration_token().await
    })).unwrap();
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(out))
}

#[get("/login")]
async fn login_ui(hb: web::Data<Arc<Handlebars<'_>>>, session: Session) -> actix_web::Result<HttpResponse> {
    if verify_session(session.clone()).is_some() {
        return Ok(HttpResponse::Found().insert_header(("Location", "/")).finish());
    }
    let error = if session.get::<String>("login_error").is_ok() {
        session.get::<String>("login_error").unwrap()
    } else {
        None
    };
    let out = hb.render("login.hbl", &json!({
        "error": error
    })).unwrap();
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(out))
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

#[post("/login")]
async fn login(form: Form<LoginForm>, store: web::Data<Store>, session: Session) -> Result<impl Responder, actix_web::Error> {
    let users = store.get::<Users>("users");
    if users.is_err() {
        session.insert("login_error", format!("Users storage has not been set up!"))?;
        return Ok(Redirect::new("/login", "/login").using_status_code(StatusCode::SEE_OTHER));
    }
    let users = users.unwrap();
    let user = users.get_user(form.username.clone());
    let user_clone = user.clone();
    if user.is_none() {
        session.insert("login_error", format!("Invalid Username!"))?;
        return Ok(Redirect::new("/login", "/login").using_status_code(StatusCode::SEE_OTHER));
    }
    if !user.unwrap().verify_password(form.password.clone()) {
        session.insert("login_error", format!("Invalid Password!"))?;
        return Ok(Redirect::new("/login", "/login").using_status_code(StatusCode::SEE_OTHER));
    }
    session.remove("login_error");
    session.insert("logged_in", true)?;
    session.insert("user", user_clone.unwrap())?;
    return Ok(Redirect::new("/login", "/").using_status_code(StatusCode::SEE_OTHER));
}

pub async fn start_web(field: Field, plugin_manager: PluginManager) -> anyhow::Result<()> {
    let mut reg = Handlebars::new();
    reg.register_template_string("index.hbl", include_str!("./templates/index.hbl"))?;
    reg.register_template_string("login.hbl", include_str!("./templates/login.hbl"))?;

    let hb = Arc::new(reg);


    let store = Store::new("data").unwrap();

    let mut users: HashMap<String, User> = HashMap::new();

    users.insert("test".to_string(), User::new("Test".to_string(), "test".to_string(), "test".to_string()));

    let users_obj = Users{
        users
    };

    store.save_with_id(&users_obj, "users").unwrap();

    tokio::spawn(HttpServer::new(move || {
            
        App::new()
        .app_data(web::Data::new(field.clone()))
        .app_data(web::Data::new(plugin_manager.clone()))
        .app_data(web::Data::new(hb.clone()))
        .app_data(web::Data::new(store.clone()))
        .wrap(SessionMiddleware::new(CookieSessionStore::default(), Key::generate()))
        .service(index)
        .service(login_ui)
        .service(login)
        .service(actix_files::Files::new("/static", Path::new(env!("CARGO_MANIFEST_DIR")).join("public/static")))
    })
    .bind(("127.0.0.1", 8080))?
    .run());
    Ok(())
}
