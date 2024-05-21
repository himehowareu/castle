use base64::{engine::general_purpose, Engine as _};
use fancy_regex::Regex;
use mlua::Lua;
use shell_words;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::{
    env,
    path::Path,
    process::{exit, Command},
};
use tera::Context;
use tera::Tera;
extern crate clap;

pub enum RenderType {
    Simple,
    Complex,
}

// fn debug(text: String) {
// println!("{text}");
// }

pub fn render(file_path: &str, files: Vec<&str>) -> String {
    let mut target_text = try_read_file(file_path);
    let tags = Regex::new(r"<(.*)>(.*)</\1>").unwrap();
    // debug(format!("{:?}", files));
    for tag in tags.captures_iter(&target_text.clone()).map(|c| c.unwrap()) {
        let tag_target = tag.get(0).unwrap().as_str();
        let tag_type = tag.get(1).unwrap().as_str();
        let tag_text: &str = tag.get(2).unwrap().as_str();
        let tag_value: String;
        // debug(format!("from : {file_path}"));
        // debug(format!("tag Type : {tag_type}"));
        // debug(format!("tag Text: {tag_text}"));
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
                    let text = render(target_file, [target_file, file_path].to_vec());
                    let _ = file.write_all(text.as_bytes());
                }
                tag_value = format!("rendered {target_file}")
            }

            _ => {
                // println!("{tag_target}::{tag_type}:{tag_text}");
                // tag_value = tag_target.to_string();
                tag_value = format!(
                    "<{tag_type}>{}</{tag_type}>",
                    render(&(&("-".to_owned() + tag_text)), files.clone())
                );

                // println!("unknown tag : {}", tag_target);
            }
        }
        // target_text = tags.replace(&target_text, tag_value).to_string();
        target_text = target_text.replacen(tag_target, &tag_value, 1);
    }
    target_text
}

fn run_command(command: &str) -> String {
    let a = shell_words::split(command);
    let mut b = a.unwrap();
    let mut out;
    if env::consts::OS == "windows" {
        let e = Command::new("cmd")
            .args(["/c", "echo", "error with command"])
            .output()
            .unwrap();
        let output = Command::new("cmd").arg("/c").args(b).output().unwrap_or(e);
        out = String::from_utf8_lossy(&output.stdout).to_string();
        if out.len() < 1 {
            out = String::from_utf8_lossy(&output.stderr).to_string();
        }
    } else {
        let e = Command::new("echo")
            .arg("error with command")
            .output()
            .unwrap();
        let output = Command::new(b.remove(0)).args(b).output().unwrap_or(e);
        out = String::from_utf8_lossy(&output.stdout).to_string();
        if out.len() < 1 {
            out = String::from_utf8_lossy(&output.stderr).to_string();
        }
    }
    rem_last(out)
}

fn rem_last(value: String) -> String {
    let mut chars = value.chars();
    chars.next_back();
    chars.as_str().to_string()
}

fn run_lua(code: &str) -> String {
    let lua = Lua::new();
    lua.load(code)
        .eval::<String>()
        .unwrap_or("lua code errored".to_string())
}

fn try_read_file(file_path: &str) -> String {
    if file_path.starts_with("-") {
        let mut chars = file_path.chars();
        chars.next();
        return chars.as_str().to_string();
    }
    if !Path::new(file_path).exists() {
        println!("target file {file_path} does not exist");
        exit(-1);
    } else {
        let mut file = File::open(file_path).unwrap();
        let mut code = String::new();
        file.read_to_string(&mut code).unwrap();
        code
    }
}

fn run_macro(file_path: &str, args: &str) -> String {
    let code = try_read_file(file_path);

    let lua = Lua::new();
    let globals = lua.globals();
    globals.set("args", args.to_string()).unwrap();
    lua.load(code).exec().unwrap();
    globals
        .get::<_, String>("output")
        .unwrap_or("error in loaded lua code : not output varible defined".into())
}

fn get_json(settings_file: &str, setting: &str) -> String {
    let content = try_read_file(settings_file);

    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    let first_name = json.get(setting).unwrap();

    first_name.to_string()
}

fn file_to_base64(file_path: &str) -> String {
    let contents = try_read_file(file_path);
    let encoded = general_purpose::STANDARD.encode(contents);
    encoded
}

fn minify(test: String) -> String {
    // https://docs.rs/minify-html/latest/minify_html/fn.minify.html
    test.replace("\n", "").replace("\'", "\"")
}

fn download_file(url: &str, file_path: &str) -> String {
    let resp = reqwest::blocking::get(url).expect("request failed");
    let body = resp.bytes().expect("body invalid");
    let _ = std::fs::write(file_path, &body);
    format!("Downloaded {file_path}")
}

fn get_file(url: &str) -> String {
    let resp = reqwest::blocking::get(url.to_string()).expect("request failed");
    let body = resp.text().expect("body invalid");
    body
}

fn delete(file_path: &str) -> String {
    fs::remove_file(file_path).unwrap();
    format!("deleted file {file_path}")
}

fn delete_folder(file_path: &str) -> String {
    fs::remove_dir_all(file_path).unwrap();
    format!("deleted folder {file_path}")
}

fn download_git(url: &str, folder_path: &str) -> String {
    git2::Repository::clone(url, folder_path).unwrap();

    format!("git {url} downloaded and saved to {folder_path} ")
}

fn run_blueprint(file_path: &str, args: &str) -> String {
    let contents = try_read_file(file_path);

    let mut context = Context::new();
    for arg in shell_words::split(args).unwrap() {
        let sep = arg.split("=").collect::<Vec<&str>>();
        context.insert(sep[0], sep[1])
    }

    let result = Tera::one_off(contents.as_str(), &context, true);

    result.unwrap_or("error in temple".to_string())
}
