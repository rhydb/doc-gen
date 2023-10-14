use std::fs::File;
use std::io::{self, prelude::*};
use std::process::exit;

use lazy_static::lazy_static;
use regex::Regex;

/*
sdoc - Simple Docs Generation for .h Files
TODO:
- HTML Header
- Change <title>
*/
const HEADER: &str = "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><link rel=\"stylesheet\" href=\"style.css\"><title>$TITLE</title></head><body>";
const FOOTER: &str = "</body></htm>";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() == 0 {
        println!("Usage: sdoc [FILE]");
        exit(0);
    }
    for arg in args {
        let file_name = &arg;
        println!("Generating docs for '{}' ...", file_name);

        let contents = match read_file(file_name.as_str()) {
            Ok(contents) => contents,
            Err(err) => {
                println!("Failed to open file {}: {}", file_name, err);
                exit(1);
            }
        };

        let index_name = format!("{}.html", file_name);
        println!("Creating index file: {}", index_name);
        let mut index = File::create(&index_name).expect(&format!("Failed to create index: {}.html", file_name));
        let mut list = format!("<h1>Functions for {}</h1>", file_name);

        let re = Regex::new(r"/\*\n(@(?:[\s\S])+?)\n*\*/\n(.+);").unwrap();
        for cap in re.captures_iter(contents.as_str()) {
            let comments = cap[1].trim();
            let function = &cap[2];
            let name = fn_name(function);
            let return_type = ret_type(function);

            match create_doc(name.as_str(), function, return_type.as_str(), comments) {
                Ok(()) => {
                    list.push_str(&format!("<li><a href=\"{}.html\">{}</a></li>", name, name));
                    println!("Created doc for {}", name)
                },
                Err(err) => println!("Error creating doc for {}: {}", name, err)
            }
        }
        index.write_all(list.as_bytes()).expect("Failed to write list to index file");
    }
}

fn read_file(file_name: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn fn_name(function: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([^*\s]+)\(").unwrap();
    }
    let result = RE.captures(function).unwrap();
    result[1].to_owned()
}

fn ret_type(function: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\s*([^*]+(?:[\S]*\s)\**).+\(.*").unwrap();
    }
    let result = RE.captures(function).unwrap();
    result[1].trim().to_owned()
}

fn create_doc(name: &str, function: &str, return_type: &str, comments: &str) -> Result<(), std::io::Error> {
    const SUBHEADING: &str = "h2";

    let mut file = File::create(format!("{}.html", name))?;

    let mut data = format!("{}<h1>{}</h1><{}>Syntax</{}><code>{}</code>", HEADER.replace("$TITLE", name), name, SUBHEADING, SUBHEADING, function);
    let mut found_param = false;

    for line in comments.lines() {
        if line.starts_with('@') {
            let (token, info) = match line.split_once(' ') {
                Some((token, info)) => (token, info),
                None => {
                    println!("Failed to split token-info for: {}", line);
                    continue;
                }
            };

            /* Parse each token and add HTML to data */
            match token {
                "@brief" | "@note" => {
                    data.push_str(format!("<p>{}</p>", markdown::to_html(info)).as_str());
                },
                "@param" => {
                    if !found_param {
                        found_param = true;
                        data.push_str(format!("<{}>Params</{}>", SUBHEADING, SUBHEADING).as_str());
                    }

                    let (param, desc) = match info.split_once(' ') {
                        Some((param, desc)) => (param, desc),
                        None => {
                            println!("@param token is invalid for: {}", line);
                            continue;
                        }
                    };

                    data.push_str(format!("<dt>{}</dt><dd>{}</dd>", param, markdown::to_html(desc)).as_str());
                },
                "@ret" | "@return" | "@retval" => {
                    data.push_str(format!("<{}>Return Value</{}><code>{}</code><p>{}</p>", SUBHEADING, SUBHEADING, return_type, markdown::to_html(&info)).as_str());
                },
                "@related" => {
                    data.push_str(format!("<{}>Related</{}><ul>", SUBHEADING, SUBHEADING).as_str());

                    for related in info.split(' ') {
                        data.push_str(format!("<li><a href=\"./{}.html\">{}</a></li>", related, related).as_str());
                    }

                    data.push_str("</ul>");
                },
                _ => {
                    println!("Unknown token: '{}'", token);
                }
            }
        }
    }
    data.push_str(FOOTER);
    file.write_all(data.as_bytes())?;
    Ok(())
}
