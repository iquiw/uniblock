use async_std::task;
use lazy_static::lazy_static;
use regex::Regex;
use surf;

const UNICODE_BLOCK_URL: &str = "http://www.unicode.org/Public/13.0.0/ucd/Blocks.txt";

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
    task::block_on(async {
        match get_unicode_blocks().await {
            Ok(s) => {
                for line in s.lines() {
                    if let Some(uni_block) = parse_line(line) {
                        println!("{}", uni_block.to_elisp());
                    }
                }
                println!("\n(provide 'unicode-block)");
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    })
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

async fn get_unicode_blocks() -> surf::Result<String> {
    surf::get(UNICODE_BLOCK_URL).recv_string().await
}

// async fn get_unicode_blocks() -> Result<String, surf::Exception> {
//     use std::io::Read;
//     use std::fs::File;
//     let mut s = String::new();
//     let mut f = File::open("unicode.txt")?;
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }
