// ---------- ---------- ---------- ---------- ---------- ---------- ---------- ----------

#[macro_use] extern crate rocket;

#[get("/<title>")]
fn title(title: &str) -> rocket::response::content::RawHtml<String> {
    let s = format!("<html> <head><title>{}</title></head> <body style=\"background-color:black; color:white\">Title: {}</body> </html>", title, title);
    rocket::response::content::RawHtml(s)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![title])
}

// ---------- ---------- ---------- ---------- ---------- ---------- ---------- ----------
