use serde::{Deserialize, Serialize};
use egui::Color32;

#[derive(Serialize, Deserialize, Debug)]
pub struct Skill {
    pub name: String,
    pub rgb: [u8; 3],
    pub text_rgb: [u8; 3],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub skills: Vec<Skill>,
}

const RAW_DATA: &str = include_str!("../data.toml");

impl Skill {
    pub fn color(&self) -> Color32 {
        Color32::from_rgb(self.rgb[0], self.rgb[1], self.rgb[2])
    }

    pub fn text_color(&self) -> Color32 {
        Color32::from_rgb(self.text_rgb[0], self.text_rgb[1], self.text_rgb[2])
    }
}

impl Default for Data {
    fn default() -> Self {
        let _self: Data = toml::from_str(RAW_DATA).expect("Failed to parse data.toml");
        _self
    }
}

impl Data {
    pub fn new() -> Self {
        log::debug!("{}", RAW_DATA);
        let _self = Data::default();
        log::debug!("Data loaded: {:?}", _self.skills);
        _self
    }

    pub fn skills(&self) -> &[Skill] {
        &self.skills
    }
}