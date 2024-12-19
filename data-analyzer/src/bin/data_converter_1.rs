use dsc_data_analyser::prelude::*;

fn main() {
    let files = read_all_exr_files_in_folder("../result/");

    convert_result(&files);
}

#[allow(dead_code)]
fn convert_result(files: &Vec<String>) {
    for file in files {
        let mut filename = file.split('/').collect::<Vec<&str>>();
        filename.insert(filename.len() - 1, "data-converter-output");

        // 去除前面形如2024-11-08_00-18-07_的时间戳
        let (_, name) = filename.last().unwrap().split_at(20);
        filename.pop();
        filename.push(name);
 
        // 简化bdpt文件名
        let tmp;
        if file.contains("bdpt") {
            tmp = filename.last().unwrap().split('_').nth(0).unwrap().to_string() + ".exr";
            filename.pop();
            filename.push(&tmp);
        }
        
        let filename = filename.join("/").to_string();
        create_directories_for_file(&filename).unwrap();

        if file.contains("sppm") {
            let data = load_data(file.as_str());

            let width = data.resolution.0;
            let height = data.resolution.1;

            save_data_simplify_sppm(&data, width, height, &filename);
        } else if file.contains("pt") || file.contains("bdpt") {
            copy_file(file.as_str(), &filename);
        } else {
            // do nothing
        }
    }
}