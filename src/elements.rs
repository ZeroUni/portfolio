use std::sync::Arc;

use egui::{TextureHandle};

pub struct Project {
    slug: String,
    title: String,
    description: String,
    tags: Vec<String>,
    thumbnail: Option<TextureHandle>, // Store the thumbnail as a TextureHandle directly
}