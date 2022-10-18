use actix_csrf::{CsrfMiddleware, extractor::{CsrfToken, CsrfGuarded, Csrf}};
use actix_web::{get, web::{self, Form}, App, HttpServer, HttpResponse, http::{header::ContentType, Method}, Responder, post};
use handlebars::Handlebars;
use rand::rngs::StdRng;
use serde_derive::Deserialize;
use serde_json::json;
use std::{sync::Arc, path::Path};

use crate::{field::Field, plugin::PluginManager};

#[get("/")]
async fn index(plugin_manager: web::Data<PluginManager>, hb: web::Data<Arc<Handlebars<'_>>>) -> actix_web::Result<HttpResponse> {
    let out = hb.render("index.hbl", &json!({
        "plugins": plugin_manager.get_plugins_metadata().await,
        "plugin_token": plugin_manager.get_plugin_registration_token().await
    })).unwrap();
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(out))
}

#[get("/login")]
async fn login_ui(hb: web::Data<Arc<Handlebars<'_>>>, token: CsrfToken) -> actix_web::Result<HttpResponse> {
    let out = hb.render("login.hbl", &json!({
        "token": token.get()
    })).unwrap();
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(out))
}

#[derive(Deserialize)]
struct LoginForm {
    csrf_token: CsrfToken,
    username: String,
    password: String,
}

impl CsrfGuarded for LoginForm {
    fn csrf_token(&self) -> &CsrfToken {
        &self.csrf_token
    }
}

#[post("/login")]
async fn login(form: Csrf<Form<LoginForm>>) -> impl Responder {
    return HttpResponse::Ok().body(form.username.clone());
}

pub async fn start_web(field: Field, plugin_manager: PluginManager) -> anyhow::Result<()> {
    let mut reg = Handlebars::new();
    reg.register_template_string("index.hbl", include_str!("./templates/index.hbl"))?;
    reg.register_template_string("login.hbl", include_str!("./templates/login.hbl"))?;

    let hb = Arc::new(reg);

    //let handlebars = |with_template| async move {render(with_template.await, hb.clone())};

    tokio::spawn(HttpServer::new(move || {
        let csrf = CsrfMiddleware::<StdRng>::new()
            .set_cookie(Method::GET, "/login");
            
        App::new()
        .app_data(web::Data::new(field.clone()))
        .app_data(web::Data::new(plugin_manager.clone()))
        .app_data(web::Data::new(hb.clone()))
        .wrap(csrf)
        .service(index)
        .service(login_ui)
        .service(login)
        .service(actix_files::Files::new("/static", Path::new(env!("CARGO_MANIFEST_DIR")).join("public/static")))
    })
    .bind(("127.0.0.1", 8080))?
    .run());
    Ok(())
}
