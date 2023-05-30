use std::{
    fs,
    env,
    path::Path
};
mod errors;
use errors::PanaceaError;

fn run() -> Result<(), PanaceaError> {
    let mut args = env::args();
    let filename = match args.nth(1) {
        Some(s) => s,
        None => {
            return Err(PanaceaError::NoFileProvided);
        }
    };
    if !Path::new(&filename).is_file() {
        return Err(PanaceaError::FileNotFound { filename })
    }
    let contents = match fs::read_to_string(&filename) {
        Ok(s) => s,
        Err(_) => return Err(PanaceaError::CannotOpen { filename })
    };

    println!("{}", contents);
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e)
    }
}
