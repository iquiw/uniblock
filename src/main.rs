extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate reqwest;

const UNICODE_BLOCK_URL: &str = "http://www.unicode.org/Public/10.0.0/ucd/Blocks.txt";

use std::io::{BufRead, BufReader, Read};

use failure::Error;
use regex::Regex;
use reqwest::StatusCode;

#[derive(Fail, Debug)]
#[fail(display = "HTTP error, status: {}", status)]
struct HttpError {
    status: StatusCode,
}

#[derive(Debug)]
struct UnicodeBlock {
    name: String,
    range: (u32, u32),
}

impl UnicodeBlock {
    fn to_elisp(&self) -> String {
        format!(
            "(defconst unicode-block-{} '(#x{:x} . #x{:x}))",
            self.name.replace(" ", "-").to_lowercase(),
            self.range.0,
            self.range.1
        )
    }
}

fn main() {
    match get_unicode_blocks() {
        Ok(inner) => {
            let r = BufReader::new(inner);
            for line in r.lines() {
                if let Some(uni_block) = parse_line(&line.unwrap()) {
                    println!("{}", uni_block.to_elisp());
                }
            }
            println!("\n(provide 'unicode-block)");
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

fn parse_line(line: &str) -> Option<UnicodeBlock> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([0-9A-F]+)\.\.([0-9A-F]+); (.*)$").unwrap();
    }
    RE.captures(&line).and_then(|c| {
        Some(UnicodeBlock {
            name: c.get(3)?.as_str().to_string(),
            range: (
                u32::from_str_radix(c.get(1)?.as_str(), 16).ok()?,
                u32::from_str_radix(c.get(2)?.as_str(), 16).ok()?,
            ),
        })
    })
}

fn get_unicode_blocks() -> Result<impl Read, Error> {
    let resp = reqwest::get(UNICODE_BLOCK_URL)?;

    if resp.status().is_success() {
        Ok(resp)
    } else {
        Err(HttpError {
            status: resp.status(),
        }.into())
    }
}

// fn get_unicode_blocks() -> Result<impl Read, Error> {
//     use std::fs::File;
//     Ok(File::open("uniblock.txt")?)
// }
