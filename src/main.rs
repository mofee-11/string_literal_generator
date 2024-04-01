use clap::{Arg, Command};
use serde_json;
use std::fs;

fn main() {
    let matches = Command::new("string_literal_generator")
        .arg(Arg::new("file").required(true))
        .get_matches();

    let file_path = match matches.get_one::<String>("file") {
        Some(t) => t,
        None => {
            eprintln!("请输入一个文件路径");
            std::process::exit(1);
        }
    };

    let file_content = match fs::read_to_string(file_path) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("无效的文件路径");
            std::process::exit(1);
        }
    };

    let json_string_literal = match serde_json::to_string(&file_content) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("json 格式解析错误，为非常见情况。请联系软件开发者");
            std::process::exit(1);
        }
    };
    println!("{}", json_string_literal);
}
