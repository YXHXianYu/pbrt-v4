use crate::Data;
use plotters::prelude::*;
use std::ops::Range;

pub fn draw(
    data: &Data,
    width: u32,
    height: u32,
    number_of_points: u32,
    filename_prefix: &str
) -> Result<(), Box<dyn std::error::Error>> {

    let width = width;
    let height = height;

    let range_x = -0.01f32..0.2f32;
    let range_y = 0.0f32..1.0f32;

    for color in ["R", "G", "B"].iter() {
        for idx in [format!("{:03}", data.spp - 1).as_str()].iter() {

            let title = format!("MSE-MSERef[{}].{}", idx, color);
            let filename = format!("{}-MSE-MSERef[{}].{}", filename_prefix, idx, color);
            let mse = format!("MSE[{}].{}", idx, color);
            let mseref = format!("MSERef[{}].{}", idx, color);

            draw_multi_channels_distribution(data, title.as_str(), filename.as_str(), Vec::from([
                (mse.as_str(), RED),
                (mseref.as_str(), GREEN),
            ]), width, height, number_of_points, range_x.clone(), range_y.clone())?;
        }
    }
    
    Ok(())
}

#[allow(non_snake_case)]
fn draw_multi_channels_distribution(
    data: &Data,
    title: &str,
    filename: &str,
    channels: Vec<(&str, RGBColor)>,
    width: u32,
    height: u32,
    number_of_points: u32,
    range_x: Range<f32>,
    range_y: Range<f32>
) -> Result<(), Box<dyn std::error::Error>> {

    for (channel_name, _) in channels.iter() {
        assert!(data.image.contains_key(*channel_name));
    }

    // println!("Start drawing {}", pathname);

    let root = BitMapBackend::new(filename, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(range_x.clone(), range_y.clone())?;

    chart
        .configure_mesh().draw()?;

    let delta = (range_x.end - range_x.start) / (number_of_points as f32 + 1_f32);

    let left_bound = (0..number_of_points)
        .map(|x| if x == 0 { range_x.start } else { range_x.start + delta * (x as f32 + 0.5_f32) })
        .collect::<Vec<f32>>();

    for (channel_name, color) in channels {
        let mut count: Vec<u32> = (0..number_of_points).map(|_| 0).collect();

        data.image[channel_name].values_as_f32().for_each(|x| {
            let mut L = 0_u32;
            let mut R = number_of_points as u32;
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
                (0..number_of_points)
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

    println!("{} is done", filename);

    Ok(())
}