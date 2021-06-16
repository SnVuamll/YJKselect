#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::path;
use std::process::Command;
use std::error::Error;
use std::env;
use std::fs;

fn main() {
    let mut file_path = file_path(env::args()).expect("file path err.");

    let config = ConfigToml::new().expect("config err.");

    match yjk_version(&mut file_path) {
        YJKversion::V2 => run(&config.yjk2_path, &file_path).expect("run error."),
        YJKversion::V3 => run(&config.yjk3_path, &file_path).expect("run error."),
    }
}

// 根据程序路径和文件路径，启动yjk打开对应工程
fn run(exe_path: &path::PathBuf, file_path: &path::PathBuf) -> Result<(), Box<dyn Error>> {
    let exe_cmdstr = exe_path.to_str().expect("exe_cmdstr failed.");
    let file_cmdstr = file_path.to_str().expect("file_cmdstr failed.");

    Command::new("cmd")
        .arg("/c")
        .arg("start")
        .arg(&exe_cmdstr)
        .arg(&file_cmdstr)
        .spawn()?;
    Ok(())
}


fn file_path(mut args: env::Args) -> Result<path::PathBuf, &'static str> {
    args.next();

    let file_path = match args.next(){
        Some(arg) => arg,
        None => return Err("Didn't get a file path."),
    };

    Ok(path::PathBuf::from(file_path))
}

fn yjk_version(file_path: &mut path::PathBuf) -> YJKversion {
    file_path.set_extension("ygt");
    if file_path.exists() {
        file_path.set_extension("yjk");
        return YJKversion::V3;
    }
    file_path.set_extension("yjk");
    YJKversion::V2
}

enum YJKversion {
    V2,
    V3,
}

struct ConfigToml {
    yjk2_path: path::PathBuf,
    yjk3_path: path::PathBuf,
}

impl ConfigToml {
    fn new() -> Result<ConfigToml, Box<dyn Error>> {
        let config_file = r".\config.toml";
        let content = fs::read_to_string(config_file)?;

        let mut yjk2_str = String::new();
        let mut yjk3_str = String::new();

        for line in content.lines() {
            if line.contains("YJK2") {
                yjk2_str = match line.split_whitespace().nth(2) {
                    Some(p) => p.to_string(),
                    None => panic!("yjk2 err"),
                };
            } else if line.contains("YJK3") {
                yjk3_str = match line.split_whitespace().nth(2) {
                    Some(p) => p.to_string(),
                    None => panic!("yjk3 err"),
                };
            }
        }

        let yjk2_path = path::PathBuf::from(yjk2_str.as_str());
        let yjk3_path = path::PathBuf::from(yjk3_str.as_str());

        Ok(ConfigToml {yjk2_path, yjk3_path})
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test() {
//         new().expect("err");
//     }
// }