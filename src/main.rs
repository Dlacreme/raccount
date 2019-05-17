extern crate serde;
extern crate crypto;
extern crate toml;
extern crate regex;
extern crate chrono;
extern crate num_bigint;
extern crate num_traits;
extern crate iso4217;

mod faker;
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

    demo().unwrap();
}

fn demo() -> Result<(), chain::error::MiningError> {
    let mut c = chain::chain::Chain::new()?;
    println!("Start");

    c.push(faker::transaction(String::from("TEST 1")))?;
    c.push(faker::transaction(String::from("TEST 2")))?;

    c.traverse();

    Ok(())
}

