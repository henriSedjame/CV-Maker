use crate::models::CV;
use docx_rs::*;

mod models;
mod errors;

fn main() {

    match CV::create("./src/cv.yaml".to_string()) {
        Ok(cv) => {
            println!("{:#?}", cv)
        }
        Err(e) => {
            println!("{:?}", e.to_string())
        }
    };

}
