use crate::{prelude::*, utilities};

pub fn save_data_rgb(data: &Data, width: usize, height: usize) {
    use exr::prelude::*;

    let r = data.image["R"].values_as_f32().collect::<Vec<f32>>();
    let g = data.image["G"].values_as_f32().collect::<Vec<f32>>();
    let b = data.image["B"].values_as_f32().collect::<Vec<f32>>();

    write_rgb_file(
        "zeroday-frame120_bdpt_512.exr",
        width,
        height,
        |x, y| {
            (
                r[(y * width + x) as usize],
                g[(y * width + x) as usize],
                b[(y * width + x) as usize]
            )
        }
    ).unwrap();
}

pub fn save_data_simplify_sppm(data: &Data, width: usize, height: usize, filename: &str) {
    use exr::prelude::*;

    let r = data.image["A1-L.R"].values_as_f32().collect::<Vec<f32>>();
    let g = data.image["A1-L.G"].values_as_f32().collect::<Vec<f32>>();
    let b = data.image["A1-L.B"].values_as_f32().collect::<Vec<f32>>();
    let bias_r = data.image["A2-Bias.R"].values_as_f32().collect::<Vec<f32>>();
    let bias_g = data.image["A2-Bias.G"].values_as_f32().collect::<Vec<f32>>();
    let bias_b = data.image["A2-Bias.B"].values_as_f32().collect::<Vec<f32>>();
    let variance_r = data.image["A3-Variance.R"].values_as_f32().collect::<Vec<f32>>();
    let variance_g = data.image["A3-Variance.G"].values_as_f32().collect::<Vec<f32>>();
    let variance_b = data.image["A3-Variance.B"].values_as_f32().collect::<Vec<f32>>();
    let spp_str = format!("{:03}", data.spp - 1);
    // println!("{}", format!("B5-MSE[{}].R", spp_str).as_str());
    let mse_r = data.image[format!("B5-MSE[{}].R", spp_str).as_str()].values_as_f32().collect::<Vec<f32>>();
    let mse_g = data.image[format!("B5-MSE[{}].G", spp_str).as_str()].values_as_f32().collect::<Vec<f32>>();
    let mse_b = data.image[format!("B5-MSE[{}].B", spp_str).as_str()].values_as_f32().collect::<Vec<f32>>();

    let pixels = SpecificChannels::build()
        .with_channel("R")
        .with_channel("G")
        .with_channel("B")
        .with_channel("bias.R")
        .with_channel("bias.G")
        .with_channel("bias.B")
        .with_channel("variance.R")
        .with_channel("variance.G")
        .with_channel("variance.B")
        .with_channel("mse.R")
        .with_channel("mse.G")
        .with_channel("mse.B")
        .with_pixel_fn(|pos| {
            let index = (pos.1 * width + pos.0) as usize;
            (
                r[index],
                g[index],
                b[index],
                bias_r[index],
                bias_g[index],
                bias_b[index],
                variance_r[index],
                variance_g[index],
                variance_b[index],
                mse_r[index],
                mse_g[index],
                mse_b[index]
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
        .to_file(filename).unwrap();

    println!("Saved to {}", filename);
    
}

pub fn save_img_delta(
    img1: &Data,
    img2: &Data,
    width: usize,
    height: usize,
    filename: &str
) {
    use exr::prelude::*;

    let n = img1.num_of_pixels as usize;
    assert!(n == img2.num_of_pixels as usize);

    let mut delta_r = vec![0.0_f32; n];
    let mut delta_g = vec![0.0_f32; n];
    let mut delta_b = vec![0.0_f32; n];

    let img1_r = img1.image.get("R").unwrap().values_as_f32().collect::<Vec<f32>>();
    let img1_g = img1.image.get("G").unwrap().values_as_f32().collect::<Vec<f32>>();
    let img1_b = img1.image.get("B").unwrap().values_as_f32().collect::<Vec<f32>>();

    let img2_r = img2.image.get("R").unwrap().values_as_f32().collect::<Vec<f32>>();
    let img2_g = img2.image.get("G").unwrap().values_as_f32().collect::<Vec<f32>>();
    let img2_b = img2.image.get("B").unwrap().values_as_f32().collect::<Vec<f32>>();

    for i in 0..n {
        delta_r[i] = (img1_r[i] - img2_r[i]).abs();
        delta_g[i] = (img1_g[i] - img2_g[i]).abs();
        delta_b[i] = (img1_b[i] - img2_b[i]).abs();
    }

    let pixels = SpecificChannels::build()
        .with_channel("R")
        .with_channel("G")
        .with_channel("B")
        .with_pixel_fn(|pos| {
            let index = (pos.1 * width + pos.0) as usize;
            (
                delta_r[index],
                delta_g[index],
                delta_b[index]
            )
        });
    
    let image = Image::from_channels(
        (width, height),
        pixels
    );

    // let mut current_progress_percentage = 0;
    let _ = utilities::create_directories_for_file(filename);
    image.write()
        .to_file(&filename).unwrap();

    println!("Saved to {}", filename);

}