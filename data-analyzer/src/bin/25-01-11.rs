use dsc_data_analyser::prelude::*;

fn main() {
    use exr::prelude::*;
    // available colors: BLACK, BLUE, CYAN, GREEN, MAGENTA, RED, TRANSPARENT, WHITE, YELLOW

    let width = 512;
    let height = 512;

    // let biased_mse = load_data("./input/25-01-11_biased_mse.exr");
    // let unbiased_mse = load_data("./input/25-01-11_unbiased_mse.exr");
    // let eta = load_data("./input/25-01-11_eta.exr");

    let biased_mse = load_data("./input/25-01-12_sppm_color.exr");
    let unbiased_mse = load_data("./input/25-01-12_pt_color.exr");

    println!("NumOfPixels: biased_mse: {}, unbiased_mse: {}", biased_mse.num_of_pixels, unbiased_mse.num_of_pixels);

    let n = biased_mse.num_of_pixels as usize;

    let mut mse_min_r = vec![0.0_f32; n];
    let mut mse_min_g = vec![0.0_f32; n];
    let mut mse_min_b = vec![0.0_f32; n];

    let b_mse_r = biased_mse.image.get("R").unwrap().values_as_f32().collect::<Vec<f32>>();
    let b_mse_g = biased_mse.image.get("G").unwrap().values_as_f32().collect::<Vec<f32>>();
    let b_mse_b = biased_mse.image.get("B").unwrap().values_as_f32().collect::<Vec<f32>>();

    let u_mse_r = unbiased_mse.image.get("R").unwrap().values_as_f32().collect::<Vec<f32>>();
    let u_mse_g = unbiased_mse.image.get("G").unwrap().values_as_f32().collect::<Vec<f32>>();
    let u_mse_b = unbiased_mse.image.get("B").unwrap().values_as_f32().collect::<Vec<f32>>();

    // let mut eta = eta.image.get(&channel.to_string()).unwrap().values_as_f32().collect::<Vec<f32>>();

    let scale = 1_f32;

    for i in 0..n {
        mse_min_r[i] = b_mse_r[i].min(u_mse_r[i] * scale);
        mse_min_g[i] = b_mse_g[i].min(u_mse_g[i] * scale);
        mse_min_b[i] = b_mse_b[i].min(u_mse_b[i] * scale);
    }

    let pixels = SpecificChannels::build()
        .with_channel("R")
        .with_channel("G")
        .with_channel("B")
        .with_pixel_fn(|pos| {
            let index = (pos.1 * width + pos.0) as usize;
            (
                mse_min_r[index],
                mse_min_g[index],
                mse_min_b[index],
            )
        });
    
    let image = Image::from_channels(
        (width, height),
        pixels
    );

    // let mut current_progress_percentage = 0;

    let filename = format!("./output/25-01-12_color.exr");

    image.write()
        .to_file(&filename).unwrap();

    println!("Saved to {}", filename);

}