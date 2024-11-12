pub mod data;
pub mod load_data;
pub mod save_data;
pub mod draw;
pub mod utilities;

use load_data::*;
use save_data::*;
use data::*;
// use draw::*;
use utilities::*;

// const INPUT_FILENAME: &str = "test_2024-11-02_10-45-41-spp16-ppi1e6-mse.max0.5f.exr";
// const INPUT_FILENAME: &str = "test_2024-11-03_22-14-11-spp16-ppi1e6-no.mse.max.exr";
// const INPUT_FILENAME: &str = "2024-11-07_16-51-20_zeroday-frame120_sppm_16-L.exr";
// const OUTPUT_FOLDER_NAME: &str = "./";
// const OUTPUT_FILE_WIDTH: u32 = 1024;
// const OUTPUT_FILE_HEIGHT: u32 = 768;
// const NUM_OF_POINTS: u32 = 100; // 该数值越大，绘制的图形越精细

fn main() {
    let files = read_all_exr_files_in_folder("../result/");

    for file in files {
        check_image_float_precision(&file);
    }

    // convert_result(&files);
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

            let width = 384;
            let height = 384;

            save_data_simplify_sppm(&data, width, height, &filename);
        } else if file.contains("pt") || file.contains("bdpt") {
            copy_file(file.as_str(), &filename);
        } else {
            // do nothing
        }
    }
}
