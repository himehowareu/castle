use base64::{engine::general_purpose, Engine as _};
use regex::Regex;
use shell_words;
use std::fs::File;
use std::io::Read;
use std::{
    env, fs,
    path::Path,
    process::{exit, Command},
};

fn run(command: &str) -> String {
    let a = shell_words::split(command);
    let mut b = a.unwrap();
    let output = Command::new(b.remove(0)).args(b).output().unwrap();
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn get_json(settings_file: &str, setting: &str) -> String {
    if !Path::new(settings_file).exists() {
        println!("settings file {settings_file} does not exist");
        exit(-1);
    }
    let file = fs::File::open(settings_file).expect("file should open read only");
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");
    let first_name = json.get(setting).unwrap();
    first_name.to_string()
}

fn file_to_base64(file_path: &str) -> String {
    if !Path::new(file_path).exists() {
        println!("target base64 file {file_path} does not exist");
        exit(-1);
    }
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let encoded = general_purpose::STANDARD.encode(contents);
    encoded
}

fn minify(test: String) -> String {
    test.replace("\n", "").replace("\'", "\"")
}

fn render(target: &str, files: Vec<&str>) -> String {
    if !Path::new(target).exists() {
        println!("target file {target} does not exist");
        exit(-1);
    }

    let includes = Regex::new(r"<include>(.*)</include>").unwrap();
    let settings = Regex::new(r"<setting>(.*)</setting>").unwrap();
    let base64s = Regex::new(r"<base64>(.*)</base64>").unwrap();
    let systems = Regex::new(r"<system>(.*)</system>").unwrap();

    let mut target_text = fs::read_to_string(target).unwrap();
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

    target_text
}

fn rem_last(value: String) -> String {
    let mut chars = value.chars();
    chars.next_back();
    chars.as_str().to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let target = &args[1];
        println!("{}", render(target, [].to_vec()));
    } else {
        println!("please pass the file you which you render as the only argument")
    }
}
