use dsc_data_analyser::prelude::*;

fn main() {
    let mse = load_data("./input/scene_sppm_s8-sppm_mse.exr");
    let mse_ref = load_data("./input/scene_sppm_s8-sppm_mse_reference.exr");
    let time_str = chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    let _ = draw_comparison_of_mse_mitsuba(&mse, &mse_ref, 100, time_str.as_str());

    let image_width = mse.resolution.0;
    let image_height = mse.resolution.1;
    let filename = format!("output/{}/mse_delta.exr", time_str);
    save_img_delta(&mse, &mse_ref, image_width, image_height, filename.as_str());
}
