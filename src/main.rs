use warden::config;

fn main() {
    let conf = config::Config::new().expect("Failed to load values from config sources.");
    println!("cfg: {:#?}", conf);
}
