use std::fs::File;
use std::io::Read;
use std::sync::OnceLock;

use async_std::task;
use regex::Regex;
use ureq;

const UNICODE_BLOCK_URL: &str = "https://www.unicode.org/Public/15.1.0/ucd/Blocks.txt";

#[derive(Debug)]
struct UnicodeBlock {
    name: String,
    range: (u32, u32),
}

impl UnicodeBlock {
    fn to_symbol(&self) -> String {
        format!(
            "unicode-block-{}",
            self.name.replace(" ", "-").to_lowercase()
        )
    }

    fn to_elisp(&self) -> String {
        format!(
            "(defconst {} '(#x{:x} . #x{:x}))",
            self.to_symbol(),
            self.range.0,
            self.range.1
        )
    }
}

fn main() {
    task::block_on(async {
        match get_unicode_blocks() {
            Ok(s) => {
                let mut v = vec![];
                for line in s.lines() {
                    if let Some(uni_block) = parse_line(line) {
                        println!("{}", uni_block.to_elisp());
                        v.push(uni_block);
                    }
                }
                print!("\n(defconst unicode-blocks\n  '(");
                for uni_block in v {
                    print!("{}\n    ", uni_block.to_symbol());
                }
                println!("))");
                match read_footer() {
                    Ok(footer) => print!("{}", footer),
                    Err(e) => eprintln!("Err: {}", e),
                }
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    })
}

fn parse_line(line: &str) -> Option<UnicodeBlock> {
    static RE_CELL: OnceLock<Regex> = OnceLock::new();
    let re = RE_CELL.get_or_init(|| Regex::new(r"^([0-9A-F]+)\.\.([0-9A-F]+); (.*)$").unwrap());
    re.captures(&line).and_then(|c| {
        Some(UnicodeBlock {
            name: c.get(3)?.as_str().to_string(),
            range: (
                u32::from_str_radix(c.get(1)?.as_str(), 16).ok()?,
                u32::from_str_radix(c.get(2)?.as_str(), 16).ok()?,
            ),
        })
    })
}

fn read_footer() -> Result<String, ureq::Error> {
    let mut s = String::new();
    let mut f = File::open("assets/footer.el")?;
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn get_unicode_blocks() -> Result<String, ureq::Error> {
    Ok(ureq::get(UNICODE_BLOCK_URL).call()?.into_string()?)
}

// async fn get_unicode_blocks() -> surf::Result<String> {
//     let mut s = String::new();
//     let mut f = File::open("unicode.txt")?;
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }
