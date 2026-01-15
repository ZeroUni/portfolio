use std::{collections::HashMap, f32::consts::PI, vec};

use egui::{include_image, panel::TopBottomSide, pos2, vec2, Align, AtomExt, Color32, Frame, Id, ImageSource, Label, Margin, Mesh, Rect, Scene, Sense, Stroke, Style, TextWrapMode, Theme, UiBuilder};
use serde::de;
use web_sys::window;

use crate::{data::{Data, ProjectHighlight, Skill}, elements::{add_highlighted_project, paint_angular_gradient, skill_frameplate, socials, ButtonWithUnderline}};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    #[serde(skip)]
    image_path: String,
    #[serde(skip)]
    scene_rect: egui::Rect,
    #[serde(skip)]
    root_url: String,
    #[serde(skip)]
    animations: HashMap<Id, (AnimateDirection, f32)>, // Map of animations by their ID, as well as their direction and progress
    #[serde(skip)]
    data: Data, // Data struct to hold skills and other data
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            image_path: "/test_img.png".to_owned(),
            scene_rect: egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(1920.0, 1080.0)),
            root_url: get_base_url(),
            animations: HashMap::new(),
            data: crate::data::Data::new(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        let dark_style = Self::get_dark_theme_style(&cc.egui_ctx);
        cc.egui_ctx.set_style_of(Theme::Dark, dark_style);
        let light_style = Self::get_light_theme_style(&cc.egui_ctx);
        cc.egui_ctx.set_style_of(Theme::Light, light_style);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    pub fn get_dark_theme_style(ctx: &egui::Context) -> Style {
        use egui::{
            style::{Selection, Visuals, Widgets},
            Color32, FontFamily, FontId, CornerRadius, Stroke, TextStyle,
        };
    
        let mut style = (*ctx.style()).clone();
    
        // Set text styles
        style.text_styles = [
            (TextStyle::Heading, FontId::new(22.0, FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(18.0, FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(16.0, FontFamily::Monospace)),
            (TextStyle::Button, FontId::new(18.0, FontFamily::Proportional)),
            (TextStyle::Small, FontId::new(14.0, FontFamily::Proportional)),
        ]
        .into();
    
        // Primary background color
        let primary_bg_color = Color32::from_rgb(16, 17, 18);
    
        // Configure visuals
        style.visuals = Visuals::dark();
        style.visuals.extreme_bg_color = primary_bg_color;
        style.visuals.override_text_color = Some(Color32::from_rgb(240, 235, 216));
        style.visuals.widgets = Widgets {
            noninteractive: egui::style::WidgetVisuals {
                bg_fill: primary_bg_color,
                bg_stroke: Stroke::new(1.0, Color32::from_gray(60)),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(240, 235, 216)),
                corner_radius: CornerRadius::same(4),
                weak_bg_fill: Color32::from_gray(32),
                expansion: 0.0,
            },
            inactive: egui::style::WidgetVisuals {
                bg_fill: Color32::from_rgb(19, 41, 61),
                bg_stroke: Stroke::new(1.0, Color32::from_rgb(22, 50, 79)),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(240, 235, 216)),
                corner_radius: CornerRadius::same(4),
                weak_bg_fill: Color32::from_gray(32),
                expansion: 0.0,
            },
            hovered: egui::style::WidgetVisuals {
                bg_fill: Color32::from_rgb(50, 50, 50),
                bg_stroke: Stroke::new(1.0, Color32::WHITE),
                fg_stroke: Stroke::new(1.0, Color32::WHITE),
                corner_radius: CornerRadius::same(4),
                weak_bg_fill: Color32::from_gray(32),
                expansion: 0.5,
            },
            active: egui::style::WidgetVisuals {
                bg_fill: Color32::from_rgb(60, 60, 60),
                bg_stroke: Stroke::new(1.0, Color32::WHITE),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(240, 235, 216)),
                corner_radius: CornerRadius::same(4),
                weak_bg_fill: Color32::from_gray(32),
                expansion: 2.0,
            },
            open: egui::style::WidgetVisuals {
                bg_fill: Color32::from_rgb(40, 40, 40),
                bg_stroke: Stroke::new(1.0, Color32::WHITE),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(240, 235, 216)),
                corner_radius: CornerRadius::same(4),
                weak_bg_fill: Color32::from_gray(32),
                expansion: 0.0,
            },
        };
    
        // Selection colors
        style.visuals.selection = Selection {
            bg_fill: Color32::from_rgb(75, 75, 75),
            stroke: Stroke::new(1.0, Color32::WHITE),
        };
    
        // Window settings
        style.visuals.window_corner_radius = CornerRadius::same(6);
        style.visuals.window_shadow = egui::Shadow {
            offset: [0, 1],
            blur: 3,
            spread: 0,
            color: Color32::from_black_alpha(128),
        };
        style.visuals.window_fill = primary_bg_color;
        style.visuals.window_stroke = Stroke::new(1.0, Color32::from_gray(60));
        style.visuals.panel_fill = primary_bg_color;
    
        // Spacing settings
        //style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        style.spacing.window_margin = egui::Margin::same(4);
        style.spacing.button_padding = egui::vec2(2.0, 2.0);
    
        style
    }

    pub fn get_light_theme_style(ctx: &egui::Context) -> Style {
        use egui::{
            style::{Selection, Visuals, Widgets},
            Color32, FontFamily, FontId, CornerRadius, Stroke, TextStyle,
        };
    
        let mut style = (*ctx.style()).clone();
    
        // Set text styles
        style.text_styles = [
            (TextStyle::Heading, FontId::new(22.0, FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(18.0, FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(16.0, FontFamily::Monospace)),
            (TextStyle::Button, FontId::new(18.0, FontFamily::Proportional)),
            (TextStyle::Small, FontId::new(14.0, FontFamily::Proportional)),
        ]
        .into();
    
        // Primary background color
        let primary_bg_color = Color32::from_rgb(202, 233, 255);
        let secondary_bg_color = Color32::from_rgb(27, 73, 101);
    
        // Configure visuals
        style.visuals = Visuals::light();
        style.visuals.extreme_bg_color = primary_bg_color;
        style.visuals.override_text_color = Some(Color32::from_rgb(33, 34, 39));
        style.visuals.widgets = Widgets {
            noninteractive: egui::style::WidgetVisuals {
                bg_fill: secondary_bg_color,
                bg_stroke: Stroke::new(1.0, Color32::from_rgb(99, 112, 116)),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(95, 168, 211)),
                corner_radius: CornerRadius::same(4),
                weak_bg_fill: Color32::from_gray(32),
                expansion: 0.0,
            },
            inactive: egui::style::WidgetVisuals {
                bg_fill: secondary_bg_color,
                bg_stroke: Stroke::new(1.0, Color32::from_rgb(99, 112, 116)),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(33, 34, 39)),
                corner_radius: CornerRadius::same(4),
                weak_bg_fill: Color32::from_rgb(190, 233, 232),
                expansion: 0.0,
            },
            hovered: egui::style::WidgetVisuals {
                bg_fill: Color32::from_rgb(170, 185, 207),
                bg_stroke: Stroke::new(1.0, Color32::from_rgb(99, 112, 116)),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(33, 34, 39)),
                corner_radius: CornerRadius::same(4),
                weak_bg_fill: Color32::from_rgb(139, 166, 169),
                expansion: 0.5,
            },
            active: egui::style::WidgetVisuals {
                bg_fill: Color32::from_rgb(60, 60, 60),
                bg_stroke: Stroke::new(1.0, Color32::from_rgb(99, 112, 116)),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(33, 34, 39)),
                corner_radius: CornerRadius::same(4),
                weak_bg_fill: Color32::from_rgb(139, 166, 169),
                expansion: 2.0,
            },
            open: egui::style::WidgetVisuals {
                bg_fill: Color32::from_rgb(40, 40, 40),
                bg_stroke: Stroke::new(1.0, Color32::from_rgb(99, 112, 116)),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(33, 34, 39)),
                corner_radius: CornerRadius::same(4),
                weak_bg_fill: Color32::from_gray(32),
                expansion: 0.0,
            },
        };
    
        // Selection colors
        style.visuals.selection = Selection {
            bg_fill: Color32::from_rgb(99, 112, 116),
            stroke: Stroke::new(1.0, Color32::from_rgb(255, 255, 255)),
        };
    
        // Window settings
        style.visuals.window_corner_radius = CornerRadius::same(6);
        style.visuals.window_shadow = egui::Shadow {
            offset: [0, 1],
            blur: 3,
            spread: 0,
            color: Color32::from_black_alpha(128),
        };
        style.visuals.window_fill = Color32::from_rgb(122, 156, 198);
        style.visuals.window_stroke = Stroke::new(1.0, Color32::from_gray(60));
        style.visuals.panel_fill = primary_bg_color;
    
        // Spacing settings
        //style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        style.spacing.window_margin = egui::Margin::same(4);
        style.spacing.button_padding = egui::vec2(2.0, 2.0);
    
        style
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        let screen_width = ctx.screen_rect().width() * ctx.zoom_factor();
        let screen_size: ScreenSize = if screen_width < 768.0 {
            ScreenSize::Small
        } else if screen_width < 1028.0 {
            ScreenSize::Medium
        } else {
            ScreenSize::Large
        };

        log::debug!("Screen size: {:?}, zoom factor: {:?}, Screen Width: {:?}", screen_size, ctx.zoom_factor(), screen_width);

        if screen_size.to_u8() == 1 && ctx.zoom_factor().ne(&(screen_width / 768.0)) {
            // Normalize screen to 768 px
            ctx.set_zoom_factor(screen_width / 768.0);
            ctx.request_repaint();
        } else if screen_size.to_u8() != 1 {
            ctx.set_zoom_factor(1.0);
        }

        let panel_location = match screen_size {
            ScreenSize::Small => TopBottomSide::Bottom,
            ScreenSize::Medium => TopBottomSide::Top,
            ScreenSize::Large => TopBottomSide::Top,
        };

        let theme_preference: egui::Theme = ctx.theme();
        let theme_text = match theme_preference {
            egui::Theme::Light => "🌖",
            egui::Theme::Dark => "🌞",
        };
        
        let menu_frame = egui::Frame {
            inner_margin: egui::Margin {
                left: 12,
                right: 14,
                top: 6,
                bottom: 8,
            },
            outer_margin: egui::Margin::same(0),
            stroke: egui::Stroke::new(1.0, ctx.style().visuals.window_stroke.color),
            fill: ctx.style().visuals.window_fill,
            ..Default::default()
        };
        egui::TopBottomPanel::new(panel_location, "top_panel").frame(menu_frame).show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.add_space(8.0);
                ui.add(
                    egui::Image::new(ImageSource::Uri(format!("{}/assets/croissant.png", &self.root_url).into())).maintain_aspect_ratio(false)
                    .fit_to_exact_size(vec2(48.0, 48.0)).corner_radius(32.0)
                );
                ui.add_space(20.0);

                let animation_value = 1.0 - self.animations.entry(Id::new("portfolio_button"))
                    .or_insert({
                        ctx.animate_value_with_time(Id::new("portfolio_button"), 0.0, 0.2); // Tell the ctx to initialize the animation with a current value of 0.0
                        (AnimateDirection::In, 0.0)
                    }).1;
                let portfolio_text = egui::RichText::new("Portfolio")
                    .font(egui::FontId::new(20.0 * (1.1 - (0.1 * animation_value)), egui::FontFamily::Proportional))
                    .color(ctx.style().visuals.override_text_color.unwrap_or(egui::Color32::WHITE));
                let test_button = ui.add(ButtonWithUnderline::new(portfolio_text).frame(false).inset([8.0 * animation_value, 8.0 * animation_value]));
                if test_button.clicked() {
                    log::info!("Portfolio button clicked");
                }
                if test_button.hovered() {
                    let (direction, progress) = self.animations.get_mut(&Id::new("portfolio_button")).unwrap();
                    // Only handle fade-in
                    if direction == &AnimateDirection::Out || *progress < 1.0 {
                        *direction = AnimateDirection::In;
                        *progress = ctx.animate_value_with_time(Id::new("portfolio_button"), 1.0, 0.2);
                    }
                } else {
                    // Handle fade-out if not hovered
                    let (direction, progress) = self.animations.get_mut(&Id::new("portfolio_button")).unwrap();
                    if *direction == AnimateDirection::In || *progress > 0.0 {
                        *direction = AnimateDirection::Out;
                        *progress = ctx.animate_value_with_time(Id::new("portfolio_button"), 0.0, 0.2);
                    }
                }
                ui.add_space(8.0);
                
                #[cfg(debug_assertions)]
                {
                    let debug_button = ui.add(ButtonWithUnderline::new(egui::RichText::new("Debug").font(egui::FontId::new(20.0, egui::FontFamily::Proportional))).frame(false).inset([8.0, 8.0]));
                    if debug_button.clicked() {
                        ctx.set_debug_on_hover(!ctx.debug_on_hover());
                    }
                }
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(10.0);
                    ui.style_mut().override_font_id = Some(egui::FontId::new(32.0, egui::FontFamily::Proportional));
                    if ui.button(theme_text).clicked() {
                        ctx.set_theme(if theme_preference == egui::Theme::Light {
                            egui::Theme::Dark
                        } else {
                            egui::Theme::Light
                        });
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let bg_painter = ctx.layer_painter(egui::LayerId::background());
            paint_angular_gradient(&bg_painter, ui.clip_rect(), egui::Color32::from_rgb(95, 15, 64), ui.visuals().extreme_bg_color, -PI / 4.0, vec2(0.4, 2.0));
            egui::Frame::group(ui.style())
                .inner_margin(egui::Margin::symmetric(18, 14))
                .outer_margin(0.0)
                .stroke(egui::Stroke::NONE)
                .show(ui, |ui| {
                    let scene: Scene = Scene::new()
                        .max_inner_size([300.0, 300.0])
                        .zoom_range(1.0..=5.0);

                    let scene_rect_snapshot = self.scene_rect.clone();
                    let scroll_area = egui::ScrollArea::both().max_width(ui.available_width()).min_scrolled_height(ui.available_height()).auto_shrink([false, false]).scroll([false, true]);


                    let scroll_response = scroll_area.show(ui, |ui| {
                        // The central panel the region left after adding TopPanel's and SidePanel's
                        ui.set_min_height(ui.available_height());
                        ui.set_width(ui.available_rect_before_wrap().width());
                        let size_horizontal = ui.clip_rect().width();

                        let main_info = Frame::group(ui.style()).stroke(Stroke::NONE);
                        let main_space = main_info.show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.add(egui::Image::new(self.root_url.to_owned() + "/assets/pride-flag.gif").fit_to_original_size(0.3));
                                ui.add_space(16.0);
                                ui.vertical(|ui| {
                                    let mut opener = egui::Frame::group(ui.style()).stroke(Stroke::NONE).fill(Color32::TRANSPARENT).inner_margin(Margin::same(4)).outer_margin(Margin::same(0)).corner_radius(2).begin(ui);
                                    {
                                        opener.content_ui.horizontal(|ui| {
                                            ui.vertical(|ui| {
                                                ui.add_space(16.0);
                                                ui.label(
                                                    egui::RichText::new("ZeroUni").font(egui::FontId::new(get_font_size(&screen_size, 4), egui::FontFamily::Proportional)).strong()
                                                );
                                            });
                                            ui.add_space(8.0);
                                            ui.vertical(|ui| {
                                                ui.label(
                                                    egui::RichText::new("Fullstack developer / backend enthusiast").font(egui::FontId::new(get_font_size(&screen_size, 1), egui::FontFamily::Proportional)).strong()
                                                );
                                                socials(ui, "github/@ZeroUni", "https://github.com/ZeroUni", &None, get_font_size(&screen_size, 1));
                                                socials(ui, "linkedin/@ZeroUni", "https://www.linkedin.com/in/ZeroUni", &None, get_font_size(&screen_size, 1));
                                            });
                                        });
                                    }
                                    let opening_rect = opener.allocate_space(ui).rect;
                                    // Paint a transparent gray gradient before painting the contents
                                    paint_angular_gradient(ui.painter(), opening_rect.expand2(vec2(8.0, 0.0)), Color32::from_rgba_unmultiplied(100, 100, 100, 50), Color32::TRANSPARENT, 1., vec2(2.0, 0.8));
                                    opener.paint(ui);

                                    ui.horizontal_wrapped(|ui| {
                                        ui.set_max_width(opening_rect.width());
                                        for skill in self.data.skills() {
                                            skill_frameplate(ui, &skill.name, skill.color(), skill.text_color(), get_font_size(&screen_size, 0));
                                        }
                                    });

                                    ui.horizontal(|ui| {
                                        ui.visuals_mut().hyperlink_color = Color32::from_rgb(128, 36, 133);
                                        ui.add(egui::github_link_file!(
                                            "https://github.com/ZeroUni/portfolio/blob/main/",
                                            "Source Code"
                                        ).open_in_new_tab(true));
                                        ui.visuals_mut().hyperlink_color = Color32::from_rgb(98, 36, 208);
                                        ui.add(egui::github_link_file!(
                                            "https://github.com/emilk/eframe_template/blob/main/",
                                            "[egui]"
                                        ).open_in_new_tab(true));
                                    });
                                });
                            });
                        }).response.rect;

                        let (highlight_space, highlight_layout) = match screen_size {
                            ScreenSize::Small | ScreenSize::Medium => (ui.allocate_rect(Rect::from_min_size(main_space.left_bottom() + vec2(0.0, 16.0), vec2(ui.available_width(), 200.0)), Sense::click()),
                            egui::Layout::top_down(egui::Align::LEFT)),
                            ScreenSize::Large => (ui.allocate_rect(Rect::from_min_size(main_space.right_top() + vec2(8.0, 0.0), vec2(ui.max_rect().width() - main_space.width() - 8.0, 200.0)), Sense::hover()),
                            egui::Layout::top_down(egui::Align::Max)),
                        };

                        ui.scope_builder(egui::UiBuilder::default().max_rect(highlight_space.rect).sense(Sense::click()).layout(highlight_layout), |ui| {
                            let outer_frame = egui::Frame::group(ui.style()).fill(Color32::from_gray(40).gamma_multiply_u8(127).blend(ui.visuals().extreme_bg_color.gamma_multiply_u8(100))).outer_margin(egui::Margin::symmetric(8, 0));
                            outer_frame.show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.set_max_width(800.0_f32.min(highlight_space.rect.width()) - 16.0);
                                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                        ui.add_space(12.0);
                                        ui.heading(egui::RichText::new("Highlights").underline());
                                    });
                                });
                                let root_url = self.root_url.to_owned();
                                ui.set_max_width(1100.0_f32.min(highlight_space.rect.width()) - 16.0);
                                let max_len = self.data.project_highlights().len() - 1;
                                for (idx, project) in self.data.project_highlights_mut().iter_mut().enumerate() {
                                    add_highlighted_project(ui, ctx, &root_url, project);
                                    ui.add_space(8.0);
                                    if idx < max_len {
                                        ui.separator();
                                        ui.add_space(8.0);
                                    }
                                }
                            });
                        });

                        let contact_frame = egui::Frame::group(ui.style())
                            .fill(Color32::from_gray(40).gamma_multiply_u8(127).blend(ui.visuals().extreme_bg_color.gamma_multiply_u8(100)))
                            .outer_margin(egui::Margin::symmetric(8, 4));
                        
                        contact_frame.show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.set_max_width(main_space.width());
                                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                    ui.heading(egui::RichText::new("Contact Me:").underline());
                                });
                                ui.hyperlink_to("[email]", "mailto:zd.muhs@gmail.com");
                            });
                        });

                        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                            powered_by_egui_and_eframe(ui);
                            egui::warn_if_debug_build(ui);
                        });

                    }).inner_rect;

                    // If the scene_rect has negative bounds (x or y), shift it to the origin preserving the size.
                    if self.scene_rect.min.x < 0.0 || self.scene_rect.min.y < 0.0 {
                        let shift = vec2(
                            self.scene_rect.min.x.min(0.0),
                            self.scene_rect.min.y.min(0.0),
                        );
                        self.scene_rect = self.scene_rect.translate(-shift);
                    }
                });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScreenSize {
    Small = 1,
    Medium = 2,
    Large = 3,
}

impl ScreenSize {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(ScreenSize::Small),
            2 => Some(ScreenSize::Medium),
            3 => Some(ScreenSize::Large),
            _ => None,
        }
    }

    fn to_u8(&self) -> u8 {
        match self {
            &ScreenSize::Small => 1,
            &ScreenSize::Medium => 2,
            &ScreenSize::Large => 3,
        }
    }

    fn as_f32(&self) -> f32 {
        self.to_u8() as f32
    }

    fn as_f64(&self) -> f64 {
        self.to_u8() as f64
    }
}

#[derive(PartialEq)]
enum AnimateDirection {
    In,
    Out,
}

pub fn get_base_url() -> String {
    window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.base_uri().ok().flatten())
        .unwrap_or_else(|| "".to_string())
}

/// Get the font size for a specific screen size and paragraph type.
/// - `screen_size`: The screen size to get the font size for.
/// - `paragraph_type`: The paragraph type to get the font size for.
///    - `0` font size for small sections
///    - `1` font size for regular text 
///    - `2` font size for subheaders
///    - `3` font size for headers / Titles
///    - `4` font size for hero text (ONE PER PAGE)
pub fn get_font_size(screen_size: &ScreenSize, paragraph_type: u8) -> f32 {
    let base_size = match screen_size {
        &ScreenSize::Small => 14.0,
        &ScreenSize::Medium => 14.0,
        &ScreenSize::Large => 16.0,
    };
    match paragraph_type {
        0 => base_size,
        1 => base_size * 1.2,
        2 => base_size * 1.4,
        3 => base_size * 1.6,
        4 => base_size * 1.8,
        _ => base_size,
    }
}