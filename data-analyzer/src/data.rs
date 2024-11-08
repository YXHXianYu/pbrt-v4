pub use std::collections::HashMap;
pub use exr::prelude::FlatSamples;

pub struct Data {
    pub image: HashMap<String, FlatSamples>,
    pub spp: u32,
    pub num_of_pixels: u32
}