use std::{fs, path::Path};

use crate::Data;

pub fn get_spp_from_all_channel_name(data: &Data) -> Option<u32> {
    for name in data.image.keys() {
        if let Some(spp) = get_spp_from_a_name(name) {
            return Some(spp);
        }
    }
    return None;
}

pub fn get_spp_from_a_name(name: &str) -> Option<u32> {
    if let Some(right) = name.split('[').nth(1) {
        if let Some(left) = right.split(']').nth(0) {
            return Some(left.parse::<u32>().unwrap());
        }
    }
    return None;
}

pub fn create_directories_for_file(file_path: &str) -> Result<(), std::io::Error> {
    // 创建一个 Path 结构体
    let path = Path::new(file_path);
    
    // 获取文件的父目录
    if let Some(parent) = path.parent() {
        // 如果父目录不存在，则创建它
        fs::create_dir_all(parent)?;
    }

    Ok(())
}

pub fn read_all_exr_files_in_folder(folder: &str) -> Vec<String> {
    use std::fs;

    let mut files = Vec::new();
    for entry in fs::read_dir(folder).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let path_str = path.to_str().unwrap();
            if path_str.ends_with(".exr") {
                files.push(path_str.to_string());
            }
        }
    }
    println!("Number of files: {}", files.len());
    println!("Files: {:#?}", files);
    files
}

pub fn copy_file(from: &str, to: &str) {
    use std::fs;
    fs::copy(from, to).unwrap();
    println!("File copied from '{}' to '{}'", from, to);
}