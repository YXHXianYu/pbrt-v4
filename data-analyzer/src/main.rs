pub mod data;
pub mod load_data;
pub mod save_data;
pub mod draw;
pub mod utilities;

use load_data::*;
use save_data::*;
use data::*;
use draw::*;
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
    // let files = read_all_exr_files_in_folder("./input/");

    // for file in files {
    //     check_image_float_precision(&file);
    // }

    // for file in files {
    //     let data = load_data(&file);

    //     let width = 1080;
    //     let height = 720;

    //    draw(&data, width, height, 100, "output/").unwrap();
    // }

    // calculate_bias_statistics(&files);

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

#[allow(dead_code)]
fn calculate_bias_statistics(files: &Vec<String>) {
    use plotters::prelude::*;
    // available colors: BLACK, BLUE, CYAN, GREEN, MAGENTA, RED, TRANSPARENT, WHITE, YELLOW

    let width = 1024;
    let height = 768;

    for file in files {
        let data = load_data(file.as_str());

        // 均值、中位数、10%~90%数值中的均值、20%~80%数值中的均值、30%~70%数值中的均值
        let mut mean = vec![0.0_f64; data.spp as usize];
        let mut mean_10_90 = vec![0.0_f64; data.spp as usize];
        let mut mean_20_80 = vec![0.0_f64; data.spp as usize];
        let mut mean_30_70 = vec![0.0_f64; data.spp as usize];
        let mut median = vec![0.0_f64; data.spp as usize];
        let mut min = vec![0.0_f64; data.spp as usize];
        let mut max = vec![0.0_f64; data.spp as usize];

        let title = format!("Bias Stats {} R", file.split('/').last().unwrap().split('.').nth(0).unwrap());

        for i in 0..data.spp as usize {
            let mut values = data.image.get(format!("B2-Bias[{:03}].R", i).as_str()).unwrap().values_as_f32().collect::<Vec<f32>>();
            values.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let len = values.len();
            let mut sum = 0.0_f64;
            let mut sum_10_90 = 0.0_f64;
            let mut sum_20_80 = 0.0_f64;
            let mut sum_30_70 = 0.0_f64;
            for j in 0..len {
                sum += values[j] as f64;
            }
            for j in (len / 10)..(len / 10 * 9) {
                sum_10_90 += values[j] as f64;
            }
            for j in (len / 10 * 2)..(len / 10 * 8) {
                sum_20_80 += values[j] as f64;
            }
            for j in (len / 10 * 3)..(len / 10 * 7) {
                sum_30_70 += values[j] as f64;
            }

            println!("{}: len [{}], sum {}, sum_10_90 {}, sum_20_80 {}, sum_30_70 {}", i, len, sum, sum_10_90, sum_20_80, sum_30_70);

            mean[i] = sum / len as f64;
            mean_10_90[i] = sum_10_90 / (len / 10 * 8) as f64;
            mean_20_80[i] = sum_20_80 / (len / 10 * 6) as f64;
            mean_30_70[i] = sum_30_70 / (len / 10 * 4) as f64;

            median[i] = values[len / 2] as f64;
            min[i] = mean[i].min(mean_10_90[i]).min(mean_20_80[i]).min(mean_30_70[i]).min(median[i]);
            max[i] = mean[i].max(mean_10_90[i]).max(mean_20_80[i]).max(mean_30_70[i]).max(median[i]);
        }

        let global_min = min.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let global_max = max.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        let output_filename = format!("BiasStats_{}_R.png", file.split('/').last().unwrap().split('.').nth(0).unwrap());
        let root = BitMapBackend::new(output_filename.as_str(), (width, height)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption(title.as_str(), ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0..data.spp, global_min..global_max).unwrap();

        chart
            .configure_mesh().draw().unwrap();

        // mean
        chart
            .draw_series(LineSeries::new(
                (0..data.spp)
                    .map(|x| (x, mean[x as usize])),
                RED,
            ))
            .unwrap()
            .label("Mean")
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));
        // mean_10_90
        chart
            .draw_series(LineSeries::new(
                (0..data.spp)
                    .map(|x| (x, mean_10_90[x as usize])),
                GREEN,
            ))
            .unwrap()
            .label("Mean_10_90")
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], GREEN));
        // mean_20_80
        chart
            .draw_series(LineSeries::new(
                (0..data.spp)
                    .map(|x| (x, mean_20_80[x as usize])),
                BLUE,
            ))
            .unwrap()
            .label("Mean_20_80")
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));
        // mean_30_70
        chart
            .draw_series(LineSeries::new(
                (0..data.spp)
                    .map(|x| (x, mean_30_70[x as usize])),
                CYAN,
            ))
            .unwrap()
            .label("Mean_30_70")
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], CYAN));
        // median
        chart
            .draw_series(LineSeries::new(
                (0..data.spp)
                    .map(|x| (x, median[x as usize])),
                MAGENTA,
            ))
            .unwrap()
            .label("Median")
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], MAGENTA));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()
            .unwrap();

        root.present().unwrap();

        println!("File '{}' processed.", output_filename);
    }
}