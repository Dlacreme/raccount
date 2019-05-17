use chrono::prelude::*;

pub fn save_file_async(chain: &super::chain::Chain) {
    let filename = generate_filename();
    match super::super::parser::chain::write(filename, chain) {
        Ok(r) => (),
        Err(err) => println!("{}", err)
    }
}

fn generate_filename() -> String {
    return format!("./raccount_{}.txt", Utc::now().timestamp());
}