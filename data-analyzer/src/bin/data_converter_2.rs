use std::collections::HashSet;

use dsc_data_analyser::prelude::*;

fn main() {
    let common_dir = "../result/";
    let input_dir = "data-converter-output/";
    let output_dir = "data-converter-output-2/";

    let input_dir = format!("{}{}", common_dir, input_dir);
    let output_dir = format!("{}{}", common_dir, output_dir);

    create_a_directory(&output_dir).unwrap();

    let files = read_all_exr_files_in_folder(&input_dir);

    convert(&files, input_dir, output_dir);
}

fn convert(files: &Vec<String>, input_dir: String, output_dir: String) {
    let mut data_set: HashSet<(String, String)> = HashSet::new();

    for file in files {
        let filename = file.split('/').last().unwrap().to_string();
        let filename_wo_suffix = filename.split('.').next().unwrap().to_string();

        let vec = filename_wo_suffix.split('_').collect::<Vec<&str>>();

        if vec.len() == 3 {
            let (scene_name, _algorithm, spp) = (vec[0], vec[1], vec[2]);

            data_set.insert((scene_name.to_string(), spp.to_string()));
        }
    }

    println!("\nData Set:");
    println!("  Length: {}", data_set.len());
    for (idx, (scene_name, spp)) in data_set.iter().enumerate() {
        println!("  Scene {} => scene_name: {}, spp: {}", idx, scene_name, spp);
    }
    println!("");

    for (scene_name, spp) in data_set.iter() {
        let data_ref = load_data(&format!("{}{}.exr", input_dir, scene_name));
        let data_pt = load_data(&format!("{}{}_pt_{}.exr", input_dir, scene_name, spp));
        let data_sppm = load_data(&format!("{}{}_sppm_{}.exr", input_dir, scene_name, spp));

        integrate_data_and_save(&data_ref, &data_pt, &data_sppm, format!("{}{}_{}.exr", output_dir, scene_name, spp));
    }

}

fn integrate_data_and_save(data_ref: &Data, data_pt: &Data, data_sppm: &Data, filename: String) {
    use exr::prelude::*;

    let width = data_ref.resolution.0;
    let height = data_ref.resolution.1;
    assert!(width == data_pt.resolution.0 && width == data_sppm.resolution.0);
    assert!(height == data_pt.resolution.1 && height == data_sppm.resolution.1);

    let reference_r = data_ref.image["R"].values_as_f32().collect::<Vec<f32>>();
    let reference_g = data_ref.image["G"].values_as_f32().collect::<Vec<f32>>();
    let reference_b = data_ref.image["B"].values_as_f32().collect::<Vec<f32>>();

    let unbiased_color_r = data_pt.image["R"].values_as_f32().collect::<Vec<f32>>();
    let unbiased_color_g = data_pt.image["G"].values_as_f32().collect::<Vec<f32>>();
    let unbiased_color_b = data_pt.image["B"].values_as_f32().collect::<Vec<f32>>();

    let biased_color_r = data_sppm.image["R"].values_as_f32().collect::<Vec<f32>>();
    let biased_color_g = data_sppm.image["G"].values_as_f32().collect::<Vec<f32>>();
    let biased_color_b = data_sppm.image["B"].values_as_f32().collect::<Vec<f32>>();

    let normal_r = data_pt.image["N.X"].values_as_f32().collect::<Vec<f32>>();
    let normal_g = data_pt.image["N.Y"].values_as_f32().collect::<Vec<f32>>();
    let normal_b = data_pt.image["N.Z"].values_as_f32().collect::<Vec<f32>>();

    let albedo_r = data_pt.image["Albedo.R"].values_as_f32().collect::<Vec<f32>>();
    let albedo_g = data_pt.image["Albedo.G"].values_as_f32().collect::<Vec<f32>>();
    let albedo_b = data_pt.image["Albedo.B"].values_as_f32().collect::<Vec<f32>>();

    let unbiased_variance_r = data_pt.image["Variance.R"].values_as_f32().collect::<Vec<f32>>();
    let unbiased_variance_g = data_pt.image["Variance.G"].values_as_f32().collect::<Vec<f32>>();
    let unbiased_variance_b = data_pt.image["Variance.B"].values_as_f32().collect::<Vec<f32>>();

    let biased_variance_r = data_sppm.image["variance.R"].values_as_f32().collect::<Vec<f32>>();
    let biased_variance_g = data_sppm.image["variance.G"].values_as_f32().collect::<Vec<f32>>();
    let biased_variance_b = data_sppm.image["variance.B"].values_as_f32().collect::<Vec<f32>>();

    let biased_bias_r = data_sppm.image["bias.R"].values_as_f32().collect::<Vec<f32>>();
    let biased_bias_g = data_sppm.image["bias.G"].values_as_f32().collect::<Vec<f32>>();
    let biased_bias_b = data_sppm.image["bias.B"].values_as_f32().collect::<Vec<f32>>();

    let pixels = SpecificChannels::build()
        .with_channel("reference.R")
        .with_channel("reference.G")
        .with_channel("reference.B")
        .with_channel("unbiased_color.R")
        .with_channel("unbiased_color.G")
        .with_channel("unbiased_color.B")
        .with_channel("biased_color.R")
        .with_channel("biased_color.G")
        .with_channel("biased_color.B")
        .with_channel("normal.R")
        .with_channel("normal.G")
        .with_channel("normal.B")
        .with_channel("albedo.R")
        .with_channel("albedo.G")
        .with_channel("albedo.B")
        .with_channel("unbiased_variance.R")
        .with_channel("unbiased_variance.G")
        .with_channel("unbiased_variance.B")
        .with_channel("biased_variance.R")
        .with_channel("biased_variance.G")
        .with_channel("biased_variance.B")
        .with_channel("biased_bias.R")
        .with_channel("biased_bias.G")
        .with_channel("biased_bias.B")
        .with_pixel_fn(|pos| {
            let index = (pos.1 * width + pos.0) as usize;
            (
                reference_r[index],
                reference_g[index],
                reference_b[index],
                unbiased_color_r[index],
                unbiased_color_g[index],
                unbiased_color_b[index],
                biased_color_r[index],
                biased_color_g[index],
                biased_color_b[index],
                normal_r[index],
                normal_g[index],
                normal_b[index],
                albedo_r[index],
                albedo_g[index],
                albedo_b[index],
                unbiased_variance_r[index],
                unbiased_variance_g[index],
                unbiased_variance_b[index],
                biased_variance_r[index],
                biased_variance_g[index],
                biased_variance_b[index],
                biased_bias_r[index],
                biased_bias_g[index],
                biased_bias_b[index]
            )
        });
    
    let image = Image::from_channels(
        (width, height),
        pixels
    );

    // let mut current_progress_percentage = 0;

    image.write()
        // .on_progress(|progress| {
        //     let new_progress = (progress * 100.0) as usize;
        //     if new_progress != current_progress_percentage {
        //         current_progress_percentage = new_progress;
        //         println!("progress: {}%", current_progress_percentage)
        //     }
        // })
        .to_file(&filename).unwrap();

    println!("Saved to {}", &filename);
    
}