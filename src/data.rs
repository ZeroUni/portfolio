use core::fmt;

use serde::{Deserialize, Serialize};
use egui::{load::{SizedTexture, TexturePoll}, Color32};

#[derive(Serialize, Deserialize, Debug)]
pub struct Skill {
    pub name: String,
    pub rgb: [u8; 3],
    pub text_rgb: [u8; 3],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub skills: Vec<Skill>,
    pub project_highlights: Vec<ProjectHighlight>,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectHighlight {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub tags: Vec<Skill>,
    thumbnail_path: String,
    pub external_link: String,
    pub highlight_imgs: Vec<String>,
    #[serde(skip)]
    pub thumbnail: Option<SizedTexture>, // Store the thumbnail as a SizedTexture directly
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
        log::debug!("Data loaded: {:?}", _self);
        _self
    }
}

impl Data {
    pub fn new() -> Self {
        Data::default()
    }

    pub fn skills(&self) -> &[Skill] {
        &self.skills
    }

    pub fn project_highlights(&self) -> &[ProjectHighlight] {
        &self.project_highlights
    }

    pub fn project_highlights_mut(&mut self) -> &mut [ProjectHighlight] {
        &mut self.project_highlights
    }
}

impl fmt::Debug for ProjectHighlight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Project Highlight")
            .field("slug", &self.slug)
            .field("title", &self.title)
            .field("description", &self.description)
            .field("tags", &self.tags)
            .field("external_link", &self.external_link)
            .field("thumbnail_path", &self.thumbnail_path)
            .finish()
    }
}

impl ProjectHighlight {
    pub fn new(
        slug: String,
        title: String,
        description: String,
        tags: Vec<Skill>,
        external_link: String,
        highlight_imgs: Vec<String>,
        thumbnail_path: String,
    ) -> Self {
        Self {
            slug,
            title,
            description,
            tags,
            external_link,
            thumbnail: None,
            highlight_imgs: highlight_imgs,
            thumbnail_path,
        }
    }

    pub fn get_set_thumbnail(&mut self, root_url: &String, ctx: &egui::Context) -> Option<SizedTexture> {
        if let Some(thumbnail) = self.thumbnail {
            Some(thumbnail)
        } else {
            let thumbnail_full_path = root_url.clone() + &self.thumbnail_path;
            let poll_result = ctx.try_load_texture(&thumbnail_full_path, Default::default(), Default::default());
            match poll_result {
                Ok(texture_poll) => {
                    match texture_poll {
                        TexturePoll::Ready { texture } => {
                            self.thumbnail = Some(texture);
                            self.thumbnail
                        },
                        TexturePoll::Pending { .. } => {
                            None
                        }
                    }
                },
                Err(e) => {
                    log::error!("Failed to load thumbnail: {}", e);
                    None
                }
            }

        }
    }
}