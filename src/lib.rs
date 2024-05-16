use base64::{engine::general_purpose, Engine as _};
use mlua::Lua;
use shell_words;
use std::fs::{self, File};
use std::io::Read;
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

pub fn run_command(command: &str) -> String {
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

pub fn rem_last(value: String) -> String {
    let mut chars = value.chars();
    chars.next_back();
    chars.as_str().to_string()
}

pub fn run_lua(code: &str) -> String {
    let lua = Lua::new();
    lua.load(code)
        .eval::<String>()
        .unwrap_or("lua code errored".to_string())
}

pub fn try_read_file(file_path: &str) -> String {
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

pub fn run_macro(file_path: &str, args: &str) -> String {
    let code = try_read_file(file_path);

    let lua = Lua::new();
    let globals = lua.globals();
    globals.set("args", args.to_string()).unwrap();
    lua.load(code).exec().unwrap();
    globals
        .get::<_, String>("output")
        .unwrap_or("error in loaded lua code : not output varible defined".into())
}

pub fn get_json(settings_file: &str, setting: &str) -> String {
    let content = try_read_file(settings_file);

    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    let first_name = json.get(setting).unwrap();

    first_name.to_string()
}

pub fn file_to_base64(file_path: &str) -> String {
    let contents = try_read_file(file_path);
    let encoded = general_purpose::STANDARD.encode(contents);
    encoded
}

pub fn minify(test: String) -> String {
    test.replace("\n", "").replace("\'", "\"")
}

pub fn download_file(url: &str, file_path: &str) -> String {
    let resp = reqwest::blocking::get(url).expect("request failed");
    let body = resp.bytes().expect("body invalid");
    let _ = std::fs::write(file_path, &body);
    format!("Downloaded {file_path}")
}

pub fn get_file(url: &str) -> String {
    let resp = reqwest::blocking::get(url.to_string()).expect("request failed");
    let body = resp.text().expect("body invalid");
    body
}

pub fn delete(file_path:&str) -> String {
    fs::remove_file(file_path).unwrap();
    format!("deleted file {file_path}")
}

pub fn delete_folder(file_path: &str) -> String {
    fs::remove_dir_all(file_path).unwrap();
    format!("deleted folder {file_path}")
}

pub fn download_git(file_path: &str, args: &str) -> String {
    let _ = Command::new("git")
        .args(&["clone", file_path, args])
        .output()
        .expect("Failed to execute command");

    // println!("git {file_path} downloaded and saved to {args} ");
    format!("git {file_path} downloaded and saved to {args} ")
}

pub fn run_blueprint(file_path: &str, args: &str) -> String {
    let contents = try_read_file(file_path);

    let mut context = Context::new();
    for arg in shell_words::split(args).unwrap() {
        let sep = arg.split("=").collect::<Vec<&str>>();
        context.insert(sep[0], sep[1])
    }

    let result = Tera::one_off(contents.as_str(), &context, true);

    result.unwrap_or("error in temple".to_string())
}

// pub fn render_with(
//     text: String,
//     select: Regex,
//     renderer: fn(&str, &&str) -> String,
//     split_type: RenderType,
// ) -> String {
//     let mut target_text = text.clone();
//     for selected in select
//         .captures_iter(target_text.clone().as_str())
//         .map(|c| c.unwrap())
//     {
//         let found = selected.get(0).unwrap().as_str();
//         let command = &selected.get(1).unwrap().as_str();
//         let include_text: String;
//         match split_type {
//             RenderType::Complex => {
//                 let sep = command.split(";").collect::<Vec<&str>>();
//                 let file_name = sep[0];
//                 let mut args = "";
//                 if sep.len() > 1 {
//                     args = sep[1];
//                 }
//                 include_text = renderer(file_name, &args);
//             }
//             RenderType::Simple => {
//                 include_text = renderer(found, &command);
//             }
//         }
//         target_text = target_text.replace(found, &include_text);
//     }
//     target_text
// }
