use std::sync::Arc;

use actix_session::Session;
use actix_web::{get, web::{self, Form}, HttpResponse, http::{header::ContentType, StatusCode}, Responder, post, route};
use actix_web_lab::web::Redirect;
use awc::Client;
use handlebars::Handlebars;
use jfs::Store;
use serde_derive::Deserialize;
use serde_json::json;

use crate::{plugin::PluginManager, store::user::{User, Users}};


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

#[get("/plugin_manager")]
async fn plugin_manager_ui(plugin_manager: web::Data<PluginManager>, hb: web::Data<Arc<Handlebars<'_>>>, session: Session) -> actix_web::Result<impl Responder> {
    if verify_session(session).is_none() {
        return Ok(HttpResponse::Found().insert_header(("Location", "/login")).finish());
    }
    let out = hb.render("plugin_manager.hbl", &json!({
        "plugins": plugin_manager.get_plugins_metadata().await,
        "plugin_token": plugin_manager.get_plugin_registration_token().await
    })).unwrap();
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(out))
}

#[route("/proxy/{plugin_id}/{tail:.*}", method = "GET")]
async fn proxy_get(plugin_manager: web::Data<PluginManager>, path: web::Path<(String, String)>, session: Session) -> actix_web::Result<impl Responder> {
    if verify_session(session).is_none() {
        return Ok(HttpResponse::Found().insert_header(("Location", "/login")).finish());
    }
    let (plugin_id, tail) = path.into_inner();
    let plugin = plugin_manager.get_plugin(plugin_id).await;
    if plugin.is_some() {
        let plugin = plugin.unwrap();
        let proxy = plugin.get_http_proxy().await;
        if proxy.is_some() {
            let proxy = proxy.unwrap();
            let client = Client::default();

            let res = client.get(proxy.generate_uri(tail)).send().await;
            if res.is_err() {
                return Ok(HttpResponse::BadGateway().finish());
            }
            let res = res.unwrap();
            let mut response = HttpResponse::build(res.status());

            res.headers().iter().for_each(|(k, v)| {
                response.insert_header((k, v.clone()));
            });

            response.streaming(res);

            return Ok(response.finish());
        } else {
            return Ok(HttpResponse::NotFound().finish());
        }
    } else {
        return Ok(HttpResponse::NotFound().finish());
    };
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