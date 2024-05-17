use fancy_regex::Regex;
use std::fs::File;
use std::io::Write;
extern crate clap;
use castle::*;

// make a tag for <build> that will run a make like system
// this will include a incermental build system along with depence resolving

fn render(file_path: &str, files: Vec<&str>) -> String {
    let mut target_text = try_read_file(file_path);
    let tags = Regex::new(r"<(.*)>(.*)</\1>").unwrap();
    for tag in tags.captures_iter(&target_text.clone()).map(|c| c.unwrap()) {
        let tag_target = tag.get(0).unwrap().as_str();
        let tag_type = tag.get(1).unwrap().as_str();
        let tag_text = tag.get(2).unwrap().as_str();
        let tag_value: String;
        // println!("{tag_target}");
        match tag_type {
            "include" => {
                if files.contains(&tag_text) {
                    println!("while rendering {file_path} found recursive include {tag_text}");
                    std::process::exit(-1)
                } else {
                    let mut files2 = files.clone();
                    files2.push(tag_text);
                    tag_value = render(tag_text, files2);
                }
            }
            "import" => {
                if files.contains(&tag_text) {
                    println!("while rendering {file_path} found recursive include {tag_text}");
                    std::process::exit(-1)
                } else {
                    let mut files2 = files.clone();
                    files2.push(tag_text);
                    tag_value = minify(render(tag_text, files2));
                }
            }
            "setting" => {
                let sep = tag_text.split(";").collect::<Vec<&str>>();
                let file_name = sep[0];
                let setting_name;
                if sep.len() > 1 {
                    setting_name = sep[1];
                } else {
                    setting_name = "";
                }
                tag_value = get_json(file_name, setting_name);
            }
            "base64" => {
                tag_value = file_to_base64(tag_text);
            }
            "system" => {
                tag_value = run_command(tag_text);
            }
            "lua" => {
                tag_value = run_lua(tag_text);
            }
            "macro" => {
                let sep = tag_text.split(";").collect::<Vec<&str>>();
                let file_name = sep[0];
                let mut args = "";
                if sep.len() > 1 {
                    args = sep[1];
                }
                tag_value = run_macro(file_name, args);
            }
            "blueprint" => {
                let sep = tag_text.split(";").collect::<Vec<&str>>();
                let file_name = sep[0];
                let mut args = "";
                if sep.len() > 1 {
                    args = sep[1];
                }
                tag_value = run_blueprint(file_name, args);
            }
            "netinclude" => {
                tag_value = get_file(tag_text);
            }
            "netimport" => {
                tag_value = minify(get_file(tag_text));
            }
            "download" => {
                let sep = tag_text.split(";").collect::<Vec<&str>>();
                let url = sep[0];
                let file_path = sep[1];
                tag_value = download_file(url, file_path);
            }
            "git" => {
                let sep = tag_text.split(";").collect::<Vec<&str>>();
                let url = sep[0];
                let file_path = sep[1];
                tag_value = download_git(url, file_path);
            }
            "delete" => {
                tag_value = delete(tag_text);
            }
            "deletefolder" => {
                tag_value = delete_folder(tag_text);
            }
            "render" => {
                let sep = tag_text.split(";").collect::<Vec<&str>>();
                let target_file = sep[0];
                let out_file = sep[1];

                if files.contains(&target_file) {
                    println!("while rendering {file_path} found recursive include {target_file}");
                    std::process::exit(-1)
                } else {
                    let mut file = File::create(out_file).unwrap();
                    let text = render(target_file, [out_file, file_path].to_vec());
                    let _ = file.write_all(text.as_bytes());
                }
                tag_value = format!("rendered {target_file}")
            }

            _ => {
                tag_value = tag_target.to_string();
                println!("unknown tag : {}", tag_target);
            }
        }
        target_text = tags.replace(&target_text, tag_value).to_string();
    }
    target_text
}

use clap::{command, Parser};

#[derive(Parser)]
#[command(author, version, about)] //, long_about = None)]
struct Args {
    #[arg()]
    pub path: String,
    #[arg()]
    pub output: Option<String>,
}
fn main() {
    let args = Args::parse();

    let file_out = args.output.unwrap_or("out.txt".into());
    let text = render(args.path.as_str(), [].to_vec());
    if file_out == "-" {
        println!("{}", text);
    } else {
        let mut file = File::create(file_out).unwrap();

        let _ = file.write_all(text.as_bytes());
    }
}
