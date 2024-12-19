use crate::prelude::*;
use std::collections::HashMap;

pub fn load_data(filepath: &str) -> Data {
    use exr::prelude::*;

    // load all channels
    let image = read().no_deep_data()
        .largest_resolution_level().all_channels().all_layers().all_attributes()
        // .on_progress(|progress| println!("progress: {:.1}", progress*100.0))
        .from_file(filepath)
        .expect(format!("run example `{}` to generate this image file", filepath).as_str());

    let mut data = HashMap::new();
    let mut spp: u32 = 0;
    let num_of_pixels: u32 = image.layer_data[0].channel_data.list[0].sample_data.len() as u32;

    let resolution = (
        image.attributes.display_window.size.0 as usize,
        image.attributes.display_window.size.1 as usize
    );
    
    for layer in &image.layer_data {
        for channel in &layer.channel_data.list {

            // match channel.sample_data.clone() as FlatSamples {
            //     FlatSamples::F16(_) => println!("Filename: {}; Float Precision: F16", filepath),
            //     FlatSamples::F32(_) => println!("Filename: {}; Float Precision: F32", filepath),
            //     FlatSamples::U32(_) => println!("Filename: {}; Float Precision: U32", filepath),
            // }

            // let channel_name = convert_channel_name(channel.name.to_string());
            let channel_name = channel.name.to_string();
            if let Some(spp_cur) = get_spp_from_a_name(&channel_name) {
                spp = spp.max(spp_cur + 1);
            }
            data.insert(channel_name, channel.sample_data.clone());
        }
    }

    // println!("File '{}' loaded. spp: '{}', num_of_pixels: '{}'", filepath, spp, num_of_pixels);

    // for (key, _) in data.iter() {
    //     println!("Channel: {}", key);
    // }

    Data {
        image: data,
        spp,
        num_of_pixels,
        resolution
    }
}

#[allow(dead_code)]
pub fn check_image_float_precision(filepath: &str) {
    use exr::prelude::*;

    // load all channels
    let image = read().no_deep_data()
        .largest_resolution_level().all_channels().all_layers().all_attributes()
        // .on_progress(|progress| println!("progress: {:.1}", progress*100.0))
        .from_file(filepath)
        .expect(format!("run example `{}` to generate this image file", filepath).as_str());

    for layer in &image.layer_data {
        for channel in &layer.channel_data.list {
            match channel.sample_data.clone() as FlatSamples {
                FlatSamples::F16(_) => println!("Filename: {}; Float Precision: F16", filepath),
                FlatSamples::F32(_) => println!("Filename: {}; Float Precision: F32", filepath),
                FlatSamples::U32(_) => println!("Filename: {}; Float Precision: U32", filepath),
            }
            return;
        }
    }
}

#[allow(dead_code)]
fn convert_channel_name(name: String) -> String {
    let v = name.split('-').nth(1);
    if let Some(v) = v {
        v.to_string()
    } else {
        name
    }
}