use rocket::*;

use rocket::fs::{FileServer, relative};

// #[get("/")]
// async fn index() -> Option<NamedFile> {
//     let mut path = Path::new(relative!("/static")).join("index.html");
//     NamedFile::open(path).await.ok()
// }

#[post("/start")]
async fn start() -> &'static str {
    send_start_command();
    "Success"
}


fn send_start_command() {
    
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![start])
        .mount("/", FileServer::from(relative!("/static")))
}