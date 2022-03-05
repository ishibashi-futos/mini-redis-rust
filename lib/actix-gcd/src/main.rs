use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

use hello::gcd;

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Server on http://localhost:3000");

    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .unwrap();
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <title>GCD Calclator</title>
        <form action="gcd" method="post">
            <input type="number" step="1" name="n"/>
            <input type="number" step="1" name="m"/>
            <button type="submit">Calculate GCD</button>
        </form>
    "#,
    )
}

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest().content_type("text/html").body(
            r#"
            <title>GCD Calclator</title>
            Invalid Input
        "#,
        );
    }

    let response = format!(
        "The gereral common divisor of {} and {} \n is <b>{}</b\n>",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}
