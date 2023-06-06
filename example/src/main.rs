use std::collections::{HashMap, HashSet};

use generic_builder::Builder;

#[derive(Builder, Debug)]
pub struct Command<T> {
    executable: String,
    #[single(arg, push)]
    args: Vec<T>,
    env: Vec<String>,
    current_dir: Option<String>,
    #[multiple(t, insert, u8, String)]
    test: HashMap<u8, String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = Command::<String>::builder()
        .executable(String::from("rm"))
        .arg(String::from("-rf"))
        .arg("/")
        .t(0, "hello")
        .env(Vec::new());
    command.build()?;
    command.build()?;
    let command = command.build_consume()?;
    println!("{:#?}", command);

    Ok(())
}
