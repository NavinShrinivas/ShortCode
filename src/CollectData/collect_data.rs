use super::datastructure::ProjDetails;
use super::datastructure::GitEnable;
use super::datastructure::GitDetails;
use std::{
    fmt::{format, Result},
    fs::{self},
    ops::RangeBounds,
    path::{Path, PathBuf},
};

fn code_lines(path: &str, proj_details: &mut ProjDetails) {
    for items in fs::read_dir(path).unwrap() {
        match items {
            Ok(e) => {
                if e.path().is_dir() {
                    code_lines(&e.path().to_str().unwrap(), proj_details);
                }
                println!("Path : {}", e.path().to_str().unwrap());
            }
            Err(_) => println!("Something went wrong"),
        }
    }
}

fn fetch_name(path: &str) -> Option<String> {
        let mut proj_name: String = String::new();
    let mut slash_count = 0; //I am llazy and don wanna use regex :/
    for chars in path.chars().rev() {
        if slash_count < 2 {
            if chars == '/' {
                slash_count += 1;
            } else {
                proj_name.push(chars);
            }
        } else {
            break;
        }
    }
    proj_name = proj_name.chars().rev().collect();
    println!("{:?}", proj_name);
    Some(proj_name)
   }


fn fetch_git_details(path: &str)->GitDetails{
    
}

fn fetch_git_status(path : &str)-> GitEnable{
let git_path: String = format!("{}{}", path, ".git");

    println!("{:?}", git_path);
  let mut folder: Vec<String> = Vec::new();
    for paths in fs::read_dir(path).unwrap() {
        match paths {
            Ok(e) => {
                folder.push(String::from(e.path().to_str().unwrap()));
            }
            _ => println!("Error !"),
        }
    }
    if folder.contains(&git_path){
        GitEnable::Present(fetch_git_details(path))
    }else{
        GitEnable::False
    }
}

pub fn collect_data(path: &str, detailstruct: &mut ProjDetails) {
    detailstruct.SetName(fetch_name(path).unwrap());
    detailstruct.set_git_staus(fetch_git_status(path))
}
