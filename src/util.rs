use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs, path::Path};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub title: String,
    pub allowed: Vec<String>,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            title: String::from(""),
            allowed: vec![],
        }
    }
}

impl Config {
    pub fn check_dir(&self, path: &str) -> bool {
        let path_str = String::from(path);
        self.allowed.contains(&path_str)
    }
}

pub fn get_dirs(dir: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let ns_repos = fs::read_dir(Path::new(dir))?
        .filter_map(|f| {
            f.ok().and_then(|d| {
                if d.path().is_dir() {
                    d.path()
                        .file_name()
                        .and_then(|n| n.to_str().map(|s| String::from(s)))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<String>>();

    Ok(ns_repos)
}

pub fn get_files(dir: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let ns_repos = fs::read_dir(Path::new(dir))?
        .filter_map(|f| {
            f.ok().and_then(|d| {
                if d.path().is_file() {
                    d.path()
                        .file_name()
                        .and_then(|n| n.to_str().map(|s| String::from(s)))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<String>>();

    Ok(ns_repos)
}

pub fn md_to_gemtext(contents: &str) -> Result<String, Box<dyn Error>> {
    let link_regex = Regex::new(r"(?x)(?P<text>\[[^\[]*\])\s*(?P<link>\([^\(]*\))")?;
    let link_strip = Regex::new(r"[!]*\[[^\[]*\]\s*\([^\(]*\)")?;
    let replace_chars = Regex::new(r"[\[\]\(\)]*")?;
    let anchor_link_regex = Regex::new(r"^#.*")?;
    let backticks_regex = Regex::new(r"^[`]{3}.*")?;
    let single_backticks_regex = Regex::new(r"[`]{1}[^`\n]{1}")?;
    // let text_links = Regex::new(r"(gemini|http(s)?)://[^\s\n]+")?;

    let by_lines: Vec<&str> = contents.split("\n").collect();

    #[derive(Debug)]
    struct Link {
        number: u32,
        link: String,
        text: String,
    }

    let mut lines: Vec<String> = vec![];
    let mut count = 1;
    for line in by_lines {
        let mut changed_line = String::from(line);
        let mut links = vec![];

        if backticks_regex.is_match(&changed_line) {
            changed_line = backticks_regex
                .replace_all(&changed_line, "```")
                .to_string();
        }

        if single_backticks_regex.is_match(&changed_line) {
            changed_line = str::replace(&changed_line, "`", "");
        }

        for link in link_regex.captures_iter(&line) {
            let link_text = replace_chars.replace_all(&link["text"], "");
            let link_uri = replace_chars.replace_all(&link["link"], "");
            let mut is_anchor_link = false;

            if anchor_link_regex.is_match(&link_uri) {
                is_anchor_link = true;
            }

            let replacement_text = match link_text.len() > 0 && !is_anchor_link {
                true => format!("{}[{}]", &link_text, count),
                false => link_text.to_string(),
            };

            changed_line = link_strip
                .replace(&changed_line, &replacement_text)
                .to_string();

            if !is_anchor_link {
                links.push(Link {
                    number: count,
                    link: link_uri.to_string(),
                    text: link_text.to_string(),
                });

                count = count + 1;
            }
        }
        // let new_line = String::from(&changed_line);
        // for text_link in text_links.captures_iter(&new_line) {
        //     let replacement_text = format!("{}[{}]", &text_link[0], count);
        //     changed_line = text_links
        //         .replace(&changed_line, &replacement_text)
        //         .to_string();

        //     links.push(Link {
        //         number: count,
        //         link: text_link[0].to_string(),
        //         text: text_link[0].to_string(),
        //     });
        //     count = count + 1;
        // }
        for l in links {
            changed_line = format!(
                "{}\r\n=> {} [{}] {}",
                changed_line, l.link, l.number, l.text
            );
        }
        lines.push(String::from(changed_line));
    }

    let contents = lines.join("\r\n");

    Ok(contents)
}
