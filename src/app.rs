use std::vec;

use egui::{include_image, panel::TopBottomSide, vec2, AtomExt, ImageSource, Scene, Style};
use web_sys::window;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    #[serde(skip)]
    image_path: String,
    #[serde(skip)]
    scene_rect: egui::Rect,
    #[serde(skip)]
    root_url: Option<String>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            image_path: "/test_img.png".to_owned(),
            scene_rect: egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(1920.0, 1080.0)),
            root_url: get_base_url(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        let style = Self::get_dark_theme_style(&cc.egui_ctx);
        cc.egui_ctx.set_style(style);

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
        style.visuals.override_text_color = Some(Color32::LIGHT_GRAY);
        style.visuals.widgets = Widgets {
            noninteractive: egui::style::WidgetVisuals {
                bg_fill: primary_bg_color,
                bg_stroke: Stroke::new(1.0, Color32::from_gray(60)),
                fg_stroke: Stroke::new(1.0, Color32::LIGHT_GRAY),
                corner_radius: CornerRadius::same(4),
                weak_bg_fill: Color32::from_gray(32),
                expansion: 0.0,
            },
            inactive: egui::style::WidgetVisuals {
                bg_fill: primary_bg_color,
                bg_stroke: Stroke::new(1.0, Color32::from_gray(75)),
                fg_stroke: Stroke::new(1.0, Color32::LIGHT_GRAY),
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
                fg_stroke: Stroke::new(1.0, Color32::WHITE),
                corner_radius: CornerRadius::same(4),
                weak_bg_fill: Color32::from_gray(32),
                expansion: 2.0,
            },
            open: egui::style::WidgetVisuals {
                bg_fill: Color32::from_rgb(40, 40, 40),
                bg_stroke: Stroke::new(1.0, Color32::WHITE),
                fg_stroke: Stroke::new(1.0, Color32::WHITE),
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
        let screen_width = ctx.screen_rect().width();
        let screen_size: ScreenSize = if screen_width < 640.0 {
            ScreenSize::Small
        } else if screen_width < 768.0 {
            ScreenSize::Medium
        } else {
            ScreenSize::Large
        };

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
            fill: ctx.style().visuals.panel_fill,
            ..Default::default()
        };
        egui::TopBottomPanel::new(panel_location, "top_panel").frame(menu_frame).show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.add_space(8.0);
                if let Some(root_url) = &self.root_url {
                    ui.add(
                        egui::Image::new(ImageSource::Uri(format!("{}/assets/croissant.png", root_url).into())).maintain_aspect_ratio(false)
                        .fit_to_exact_size(vec2(48.0, 48.0)).corner_radius(32.0)
                    );
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
            egui::Frame::group(ui.style())
                .inner_margin(egui::Margin::symmetric(18, 14))
                .outer_margin(0.0)
                .stroke(egui::Stroke::NONE)
                .show(ui, |ui| {
                    let scene: Scene = Scene::new()
                        .max_inner_size([300.0, 300.0])
                        .zoom_range(1.0..=5.0);

                    let scene_rect_snapshot = self.scene_rect.clone();
                    let scroll_area = egui::ScrollArea::both().max_width(ui.available_width() - 20.0).min_scrolled_height(ui.available_height()).auto_shrink([false, false]).scroll([false, true]);


                    scroll_area.show(ui, |ui| {
                            // The central panel the region left after adding TopPanel's and SidePanel's
                            ui.set_min_height(ui.available_height());
                            ui.set_width(ui.available_rect_before_wrap().width());
                            let left = ui.min_rect().left();
                            let top = ui.min_rect().top();
                            let size_horizontal = ui.clip_rect().width();
                            let label = egui::Label::new(egui::RichText::new("This should show at the top left").font(egui::FontId::new(22.0, egui::FontFamily::Proportional))).halign(egui::Align::LEFT);
                            let mut top_offset = top;
                            let left_offset = left;
                            let title = ui.put(
                                egui::Rect::from_min_size(
                                    egui::pos2(left_offset, top_offset),
                                    vec2(250.0_f32.min(size_horizontal / 2.0).max(100.0), 10.0),
                                ),
                                label,
                            );
                            top_offset += title.rect.height() + 5.0;

                            // ui.label(format!("{:#?}", ui.clip_rect())); 
                            // ui.label(format!("{:#?}", scene_rect_snapshot));

                            let debug_layout = egui::Label::new(format!("Scene Rect: {:#?}\nClip Rect: {:#?}", scene_rect_snapshot, ui.clip_rect())).halign(egui::Align::LEFT).extend();
                            let debug_preferred_size = debug_layout.layout_in_ui(ui).2.rect.width();

                            let rect_debugs = ui.put(
                                egui::Rect::from_min_size(
                                    egui::pos2(left_offset, top_offset),
                                    vec2(debug_preferred_size.clamp(100.0, size_horizontal / 2.0), 10.0),
                                ),
                                egui::Label::new(format!("Scene Rect: {:#?}\nClip Rect: {:#?}", scene_rect_snapshot, ui.clip_rect())).halign(egui::Align::LEFT),
                            );

                            ui.horizontal(|ui| {
                                ui.set_max_size(vec2(500.0_f32.min(size_horizontal / 2.0).max(200.0), 100.0));
                                ui.label("Write something: ");
                                ui.text_edit_singleline(&mut self.label);
                            });

                            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
                            if ui.button("Increment").clicked() {
                                self.value += 1.0;
                            }

                            ui.put(
                                egui::Rect::from_min_size(
                                    egui::pos2(left_offset, top + title.rect.height() + rect_debugs.rect.height() + 10.0),
                                    vec2(size_horizontal, 1.0),
                                ),
                                egui::Separator::default().spacing(size_horizontal),
                            );

                            ui.add(egui::github_link_file!(
                                "https://github.com/emilk/eframe_template/blob/main/",
                                "Source code."
                            ));

                            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                                powered_by_egui_and_eframe(ui);
                                egui::warn_if_debug_build(ui);
                            });

                        });
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

enum ScreenSize {
    Small,
    Medium,
    Large,
}

pub fn get_base_url() -> Option<String> {
    window().and_then(|win| win.location().origin().ok())
}