extern crate serde;
extern crate crypto;
extern crate serde_derive;
extern crate serde_json;
extern crate toml;
extern crate regex;
extern crate chrono;
extern crate num_bigint;
extern crate num_traits;

mod config;
mod parser;
mod chain;
mod transaction;

fn hello_world(conf: &config::Config) {
    println!("{} - v{} running on {} mode", conf.app.name, conf.app.version.to_string(),
        if conf.app.prod { String::from("prod") } else { String::from("debug") });
}

fn main() {
    let conf = config::Config::load(String::from("./Raccount.toml"));
    hello_world(&conf);

    demo();
}

fn demo() -> Result<(), chain::error::MiningError> {
    let mut c = chain::chain::Chain::new()?;
    println!("Start");
    c.push("New block Hello");
    c.push("Print block haha");
    c.push("wowowow hello");

    c.traverse();

    Ok(())
}

