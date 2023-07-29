use rocket::{ get, routes, Build };
#[macro_use] extern crate rocket;
use rand::seq::SliceRandom;
use rand::Rng;

#[get("/random_number")]
fn random_number_handler() -> String {
    let number: u32 = rand::thread_rng().gen_range(1..=100);
    return format!("{}", number);
}

#[get("/random_color")]
fn random_color_handler() -> String {
    let mut rng = rand::thread_rng();
    let r: u8 = rng.gen();
    let g: u8 = rng.gen();
    let b: u8 = rng.gen();
    return format!("#{:02X}{:02X}{:02X}", r, g, b);
}

#[get("/random_id")]
fn random_id_handler() -> String {
    let id: u64 = rand::thread_rng().gen();
    return format!("{}", id);
}

#[get("/random_code")]
fn random_code_handler() -> String {
    let char_set: Vec<char> = ('A'..='Z').chain('a'..='z').collect();
    let code: String = (0..12)
        .map(|_| char_set.choose(&mut rand::thread_rng()).unwrap())
        .collect();

    return code;
}

#[launch]
fn rocket() -> rocket::Rocket<Build> {
    rocket::build()
        .mount("/", routes![
            random_number_handler,
            random_color_handler,
            random_id_handler,
            random_code_handler
        ])
}