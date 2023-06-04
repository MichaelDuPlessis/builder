use std::collections::HashSet;

use generic_builder::Builder;

#[derive(Builder, Debug)]
pub struct Command {
    executable: String,
    #[single(arg, push)]
    args: Vec<String>,
    env: Vec<String>,
    #[single(current_dir, insert)]
    current_dir: Option<HashSet<String>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = Command::builder()
        .executable(String::from("rm"))
        .arg(String::from("-rf"))
        .arg("/")
        .env(Vec::new())
        .current_dir("test")
        .build()?;

    println!("{:#?}", command);

    Ok(())
}
