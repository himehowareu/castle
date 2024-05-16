use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::process::exit;
extern crate clap;
use castle::*;



fn render(target: &str, files: Vec<&str>) -> String {
    let mut target_text = try_read_file(target);

    let includes = Regex::new(r"<include>(.*)</include>").unwrap();
    let settings = Regex::new(r"<setting>(.*)</setting>").unwrap();
    let base64s = Regex::new(r"<base64>(.*)</base64>").unwrap();
    let systems = Regex::new(r"<system>(.*)</system>").unwrap();
    let luas = Regex::new(r"<lua>(.*)</lua>").unwrap();
    let macros = Regex::new(r"<macro>(.*)</macro>").unwrap();
    let blueprints = Regex::new(r"<blueprint>(.*)</blueprint>").unwrap();
    let netincludes = Regex::new(r"<netinclude>(.*)</netinclude>").unwrap();
    let downloads = Regex::new(r"<download>(.*)</download>").unwrap();
    let gits = Regex::new(r"<git>(.*)</git>").unwrap();
    let deletes = Regex::new(r"<delete>(.*)</delete>").unwrap();
    let deletefolders = Regex::new(r"<deletefolder>(.*)</deletefolder>").unwrap();

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

    target_text = render_with(target_text, settings, get_json, RenderType::Complex);
    target_text = render_with(target_text, luas, run_lua, RenderType::Simple);
    target_text = render_with(target_text, base64s, file_to_base64, RenderType::Simple);
    target_text = render_with(target_text, macros, run_macro, RenderType::Complex);
    target_text = render_with(target_text, blueprints, run_blueprint, RenderType::Complex);
    target_text = render_with(target_text, netincludes, get_file, RenderType::Simple);
    target_text = render_with(target_text, downloads, download_file, RenderType::Complex);
    target_text = render_with(target_text, gits, download_git, RenderType::Complex);
    target_text = render_with(target_text, systems, run, RenderType::Simple);
    target_text = render_with(target_text, deletes, delete, RenderType::Simple);
    target_text = render_with(target_text, deletefolders, delete_folder, RenderType::Simple);


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
