pub mod ankify;
pub mod parser;
use std::env;
use std::path::PathBuf;
use std::{env, error::Error, fs};

use pdf::build::*;
use pdf::content::*;
use pdf::error::PdfError;
use pdf::file::File;
use pdf::object::*;

pub fn parse_pdf(pdf_path: String) -> Result<(), PdfError> {

    let mut file = File::open(pdf_path) 
        .expect("Failed to open file");
    
    let mut pages = Vec::new();

    for page in file.pages() {
        let page = page?;
        let content = page.contents()?;
        let content = content.decode()?;
        let content = content.to_text()?;
        pages.push(content);
    }

    println!("{:?}", pages);
}


#[cfg(test)]
mod tests {
    use super::*;   

    #[test]
    fn print_pages() {
        let pdf_path = "test.pdf";
        let result = parse_pdf(pdf_path.to_string());
        assert_eq!(result, Ok(()));
    }
}
