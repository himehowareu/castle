use base64::{engine::general_purpose, Engine as _};
use mlua::Lua;
use regex::Regex;
use shell_words;
use std::fs::File;
use std::io::{Read, Write};
use std::{
    env, fs,
    path::Path,
    process::{exit, Command},
};
use tera::Context;
use tera::Tera;
extern crate clap;

fn run(command: &str) -> String {
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
    out
}

fn run_lua(code: &str) -> String {
    let lua = Lua::new();
    lua.load(code)
        .eval::<String>()
        .unwrap_or("lua code errored".to_string())
}

fn try_read_file(file_path: &str) -> String {
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
    globals.set("args", args).unwrap();
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
    test.replace("\n", "").replace("\'", "\"")
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

fn render(target: &str, files: Vec<&str>) -> String {
    let mut target_text = try_read_file(target);

    let includes = Regex::new(r"<include>(.*)</include>").unwrap();
    let settings = Regex::new(r"<setting>(.*)</setting>").unwrap();
    let base64s = Regex::new(r"<base64>(.*)</base64>").unwrap();
    let systems = Regex::new(r"<system>(.*)</system>").unwrap();
    let luas = Regex::new(r"<lua>(.*)</lua>").unwrap();
    let macros = Regex::new(r"<macro>(.*)</macro>").unwrap();
    let blueprints = Regex::new(r"<blueprint>(.*)</blueprint>").unwrap();

    for include in includes
        .captures_iter(target_text.clone().as_str())
        .map(|c| c.extract::<1>())
    {
        let found = include.0;
        let name = &include.1[0];
        if files.contains(name) {
            println!("while rendering {target} found recursive include {name}");
            exit(-1)
        } else {
            let mut files2 = files.clone();
            files2.push(include.0);
            let include_text = minify(render(name, files2));
            target_text = target_text.replace(found, &include_text);
        }
    }

    for setting in settings
        .captures_iter(target_text.clone().as_str())
        .map(|c| c.extract::<1>())
    {
        let found = setting.0;
        let set = &setting.1[0];
        let sep = set.split(":").collect::<Vec<&str>>();
        let file_name = sep[0];
        let setting_name = sep[1];
        let include_text = get_json(file_name, setting_name);
        target_text = target_text.replace(found, &include_text);
    }

    for b64 in base64s
        .captures_iter(target_text.clone().as_str())
        .map(|c| c.extract::<1>())
    {
        let found = b64.0;
        let file_name = &b64.1[0];
        let include_text = file_to_base64(file_name);
        target_text = target_text.replace(found, &include_text);
    }

    for system in systems
        .captures_iter(target_text.clone().as_str())
        .map(|c| c.extract::<1>())
    {
        let found = system.0;
        let command = &system.1[0];
        let include_text = rem_last(run(command));
        target_text = target_text.replace(found, &include_text);
    }

    for lua in luas
        .captures_iter(target_text.clone().as_str())
        .map(|c| c.extract::<1>())
    {
        let found = lua.0;
        let code = &lua.1[0];
        let include_text = run_lua(code);
        target_text = target_text.replace(found, &include_text);
    }

    for mac in macros
        .captures_iter(target_text.clone().as_str())
        .map(|c| c.extract::<1>())
    {
        let found = mac.0;
        let set = &mac.1[0];
        let sep = set.split(":").collect::<Vec<&str>>();
        let file_name = sep[0];
        let mut macro_args = "";
        if sep.len() > 1 {
            macro_args = sep[1];
        }
        let include_text = run_macro(file_name, macro_args);
        target_text = target_text.replace(found, &include_text);
    }

    for blueprint in blueprints
        .captures_iter(target_text.clone().as_str())
        .map(|c| c.extract::<1>())
    {
        let found = blueprint.0;
        let set = &blueprint.1[0];
        let sep = set.split(":").collect::<Vec<&str>>();
        let file_name = sep[0];
        let blueprint_args = sep[1];
        let include_text = run_blueprint(file_name, blueprint_args);
        target_text = target_text.replace(found, &include_text);
    }

    target_text
}

fn rem_last(value: String) -> String {
    let mut chars = value.chars();
    chars.next_back();
    chars.as_str().to_string()
}

use clap::{command, Parser};

#[derive(Parser)]
#[command(author, version, about)]//, long_about = None)]
struct Args {
    #[arg()]
    pub path: String,
    #[arg()]
    pub output: Option<String>,
}
fn main() {
    let args = Args::parse();

    // println!("Rendering, {}!", args.path);
    // println!("{}", render(args.path.as_str(), [].to_vec()));
    // println!("{:?}",args.output.unwrap_or("out.txt".into()))

    let file_out = args.output.unwrap_or("out.txt".into());
    let text = render(args.path.as_str(), [].to_vec());

    if file_out == "-" {
        println!("{}", text);
    } else {
        let mut file = File::create(file_out).unwrap();

        let _ = file.write_all(text.as_bytes());
    }
}
