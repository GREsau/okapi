use rocket_lamb::RocketExt;

fn main() {
    json_web_api::make_rocket().lambda().launch();
}
