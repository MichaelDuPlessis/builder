use std::collections::{HashMap, HashSet};

use generic_builder::Builder;

#[derive(Builder, Debug)]
pub struct Command {
    executable: String,
    #[single(arg, push)]
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = Command::builder()
        .executable(String::from("rm"))
        .arg(String::from("-rf"))
        .arg("/")
        .env(Vec::new());
        .build_consume()?;

    println!("{:#?}", command);

    Ok(())
}
