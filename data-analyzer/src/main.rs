extern crate exr;

use plotters::prelude::*;
use exr::prelude::FlatSamples;
use std::{collections::HashMap, ops::Range};

#[allow(dead_code)]
struct Data {
    image: HashMap<String, FlatSamples>,
    spp: u32,
    num_of_pixels: u32
}

const INPUT_FILENAME: &str = "test_2024-11-02_10-45-41-spp16-ppi1e6-mse.max0.5f.exr";
// const INPUT_FILENAME: &str = "test_2024-11-03_22-14-11-spp16-ppi1e6-no.mse.max.exr";
const OUTPUT_FOLDER_NAME: &str = "output";
const OUTPUT_FILE_WIDTH: u32 = 1024;
const OUTPUT_FILE_HEIGHT: u32 = 768;
const NUM_OF_POINTS: u32 = 100; // 该数值越大，绘制的图形越精细

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = load_data(INPUT_FILENAME);
    draw(&data)
}

fn load_data(filepath: &str) -> Data {
    use exr::prelude::*;

    // // load metadata
    // let meta_data = MetaData::read_from_file(
    //     filepath,
    //     false // do not throw an error for invalid or missing attributes, skipping them instead
    // ).expect(format!("run example `{}` to generate this image file", filepath).as_str());

    // for (layer_index, image_layer) in meta_data.headers.iter().enumerate() {
    //     println!(
    //         "custom meta data of layer #{}:\n{:#?}",
    //         layer_index, image_layer.own_attributes
    //     );
    // }

    // load all channels
    let image = read().no_deep_data()
        .largest_resolution_level().all_channels().all_layers().all_attributes()
        .on_progress(|progress| println!("progress: {:.1}", progress*100.0))
        .from_file(filepath)
        .expect(format!("run example `{}` to generate this image file", filepath).as_str());

    let mut data = HashMap::new();
    let mut spp: u32 = 0;
    let num_of_pixels: u32 = image.layer_data[0].channel_data.list[0].sample_data.len() as u32;
    
    for layer in &image.layer_data {
        for channel in &layer.channel_data.list {
            let channel_name = convert_channel_name(channel.name.to_string());
            if let Some(spp_cur) = get_content_in_2_char(&channel_name, '[', ']') {
                spp = spp.max(spp_cur + 1);
            }
            data.insert(channel_name, channel.sample_data.clone());
        }
    }

    println!("SPP: {}", spp);
    println!("Number of pixels: {}", num_of_pixels);

    // for (key, _) in data.iter() {
    //     println!("Channel: {}", key);
    // }

    Data{image: data, spp, num_of_pixels}
}

fn get_content_in_2_char(name: &String, left_c: char, right_c: char) -> Option<u32> {
    if let Some(right) = name.split(left_c).nth(1) {
        if let Some(left) = right.split(right_c).nth(0) {
            println!("{}", left);
            Some(left.parse::<u32>().unwrap())
        } else {
            None
        }
    } else {
        None
    }
}

fn convert_channel_name(name: String) -> String {
    name.split('-').nth(1).unwrap().to_string()
}

fn draw(data: &Data) -> Result<(), Box<dyn std::error::Error>> {

    let width = OUTPUT_FILE_WIDTH;
    let height = OUTPUT_FILE_HEIGHT;

    let range_x = -0.01f32..0.5f32;
    let range_y = 0.0f32..1.0f32;

    for color in ["R"].iter() {
        for idx in ["000", "001", format!("{:03}", data.spp - 1).as_str()].iter() {

            let title = format!("MSE-MSERef-[{}].{}", idx, color);
            let mse = format!("MSE[{}].{}", idx, color);
            let mseref = format!("MSERef[{}].{}", idx, color);

            draw_multi_channels_distribution(data, title.as_str(), Vec::from([
                (mse.as_str(), RED),
                (mseref.as_str(), GREEN),
            ]), width, height, range_x.clone(), range_y.clone())?;
        }
    }
    
    Ok(())
}

#[allow(non_snake_case)]
fn draw_multi_channels_distribution(data: &Data, title: &str, channels: Vec<(&str, RGBColor)>, width: u32, height: u32, range_x: Range<f32>, range_y: Range<f32>) -> Result<(), Box<dyn std::error::Error>> {

    if title.contains("/") {
        panic!("Title cannot contain '/'");
    }
    for (channel_name, _) in channels.iter() {
        assert!(data.image.contains_key(*channel_name));
    }

    let filename = format!("{} - distribution", title);
    let pathname = format!("{}/{}.png", OUTPUT_FOLDER_NAME, filename);

    // println!("Start drawing {}", pathname);

    let root = BitMapBackend::new(pathname.as_str(), (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(filename.as_str(), ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(range_x.clone(), range_y.clone())?;

    chart
        .configure_mesh().draw()?;

    let delta = (range_x.end - range_x.start) / (NUM_OF_POINTS as f32 + 1_f32);

    let left_bound = (0..NUM_OF_POINTS)
        .map(|x| if x == 0 { range_x.start } else { range_x.start + delta * (x as f32 + 0.5_f32) })
        .collect::<Vec<f32>>();

    for (channel_name, color) in channels {
        let mut count: Vec<u32> = (0..NUM_OF_POINTS).map(|_| 0).collect();

        data.image[channel_name].values_as_f32().for_each(|x| {
            let mut L = 0_u32;
            let mut R = NUM_OF_POINTS as u32;
            while R - L > 1 {
                let mid = (L + R) / 2;
                if x >= left_bound[mid as usize] {
                    L = mid;
                } else {
                    R = mid;
                }
            }
            count[L as usize] += 1;
        });

        chart
            .draw_series(LineSeries::new(
                (0..NUM_OF_POINTS)
                    .map(|x| (
                        range_x.start + delta * (x as f32 + 1_f32),          // x
                        count[x as usize] as f32 / data.num_of_pixels as f32 // y
                    )),
                color.clone(),
            ))?
            .label(channel_name)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color.clone()));
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    println!("{} is done", pathname);

    Ok(())
}