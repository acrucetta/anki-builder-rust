use std::path::PathBuf;
use std::{env, error::Error, fs};

use pdf::build::*;
use pdf::content::*;
use pdf::error::PdfError;
use pdf::file::File;
use pdf::object::*;

pub fn parse_pdf(pdf_path: &String) -> Result<(), PdfError> {
    // Open the PDF file
    let file = File::open(pdf_path)?;
    let mut pages = Vec::new();
    
    for page in file.pages().take(1) {
        let page = page.unwrap();
        if let Some(ref _contents) = page.contents {
            println!("Page: {:?}", page);
        }
        let mut new_page = PageBuilder::from_page(&page)?;
        for s in new_page.content.as_mut().iter_mut().flat_map(|c| c.parts.iter_mut()) {
            *s = Stream::new((), s.data(&file)?);
            println!("Stream: {:?}", s)
        }
        pages.push(new_page);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_pages() {
        // Print current working directory
        let pdf_path = "data/test.pdf";
        let result = parse_pdf(&pdf_path.to_string());
        assert_eq!(result.is_ok(), true);
    }
}
