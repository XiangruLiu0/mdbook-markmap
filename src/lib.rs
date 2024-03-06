use base64::Engine;
use mdbook::{
    book::{Book, BookItem},
    errors::Error as MdbookError,
    preprocess::{Preprocessor, PreprocessorContext},
};
use regex::Regex;
use std::{io::Write, process::Command};
use tempfile::{self, NamedTempFile};
use tracing::debug;

type MdbookResult<T> = Result<T, MdbookError>;

pub struct MarkmapPreprocessor;

impl Preprocessor for MarkmapPreprocessor {
    fn name(&self) -> &str {
        "markmap"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> MdbookResult<Book> {
        book.for_each_mut(|section| {
            if let BookItem::Chapter(chapter) = section {
                debug!(chapter_name = chapter.name, "Processing chapter");
                let content = &chapter.content;
                match Self::process_markmap_blocks(content) {
                    Ok(new_content) => chapter.content = new_content,
                    Err(e) => eprintln!("Error processing markmap blocks: {}", e),
                }
            }
        });
        Ok(book)
    }
}

impl MarkmapPreprocessor {
    fn process_markmap_blocks(content: &str) -> MdbookResult<String> {
        let re = Regex::new(r"```markmap\n([\s\S]*?)\n```").unwrap();
        let new_content = re
            .replace_all(content, |caps: &regex::Captures| {
                let markmap_content = &caps[1];
                match Self::generate_markmap_html(markmap_content) {
                    Ok(tmp_file) => {
                        let content = std::fs::read_to_string(tmp_file.path()).unwrap();
                        // encode with base64
                        let base64_content =
                            base64::engine::general_purpose::STANDARD.encode(content.as_bytes());
                        let uri = format!("data:text/html;base64,{}", base64_content);
                        // return the HTML link to the output file
                        format!("<embed src=\"{}\" width=\"100%\"></embed>", uri)
                    }
                    // Ok(svg) => svg,
                    Err(_) => caps[0].to_string(), // In case of an error, return the original text
                }
            })
            .to_string();
        Ok(new_content)
    }

    fn generate_markmap_html(input: &str) -> MdbookResult<NamedTempFile> {
        // create a temporary file to store the markmap input
        let mut input_file =
            tempfile::NamedTempFile::new().expect("Failed to create temporary file");
        debug!(?input_file, "Writing markmap input to temporary file");
        input_file
            .write_all(input.as_bytes())
            .expect("Failed to write input to file");
        input_file.flush().expect("Failed to flush input file");
        // create a temporary file to store output SVG
        let output_file = tempfile::NamedTempFile::new().expect("Failed to create temporary file");
        debug!(?output_file, "Creating temporary file for SVG output");
        // Example of calling an external command, replace "markmap-cli" with your actual command
        let output = Command::new("markmap")
            .arg("-o")
            .arg(output_file.path())
            .arg(input_file.path())
            .output()
            .expect("Failed to execute command");

        if !output.status.success() {
            return Err(MdbookError::msg(format!(
                "Failed to generate SVG from markmap, output: {}",
                String::from_utf8(output.stderr).unwrap(),
            )));
        }
        Ok(output_file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_markmap_blocks() {
        let input = r#"# Hello

```markmap

# First

## Second

### Third

```

Out of markmap block
"#;

        let new_content = MarkmapPreprocessor::process_markmap_blocks(input).unwrap();
        println!("{:?}", new_content);
    }

    #[test]
    fn test_generate_svg_from_markmap() {
        let input = r#"

# Hello

## First
## Second
"#;

        let svg = MarkmapPreprocessor::generate_markmap_html(input);
        println!("{:?}", svg);
    }
}
