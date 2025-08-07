use std::sync::Arc;

use egui::{text_selection::visuals, Atom, AtomKind, AtomLayout, AtomLayoutResponse, Color32, CornerRadius, Frame, Image, IntoAtoms, Margin, Response, Sense, Stroke, TextWrapMode, TextureHandle, Ui, UiBuilder, Vec2, Widget, WidgetInfo, WidgetText, WidgetType};
use web_sys::{window, Url};

pub struct Project {
    slug: String,
    title: String,
    description: String,
    tags: Vec<String>,
    thumbnail: Option<TextureHandle>, // Store the thumbnail as a TextureHandle directly
}

#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct ButtonWithUnderline<'a> {
    color: Color32,
    underline_color: Option<Color32>, // Inherets color from the button if not specified
    layout: AtomLayout<'a>,
    fill: Option<Color32>,
    stroke: Option<Stroke>,
    frame: Option<bool>,
    frame_when_inactive: bool,
    min_size: Vec2,
    corner_radius: Option<CornerRadius>,
    selected: bool,
    inset: Vec2,
    hover_inset: Vec2,
}

impl<'a> ButtonWithUnderline<'a> {
    pub fn new(atoms: impl IntoAtoms<'a>) -> Self {
        Self {
            layout: AtomLayout::new(atoms.into_atoms()).sense(Sense::click()),
            fill: None,
            stroke: None,
            frame: None,
            frame_when_inactive: true,
            min_size: Vec2::ZERO,
            corner_radius: None,
            selected: false,
            color: Color32::from_black_alpha(0),
            underline_color: None,
            inset: Vec2::ZERO,
            hover_inset: Vec2::splat(-2.0),
        }
    }

    /// Show a selectable button.
    ///
    /// Equivalent to:
    /// ```rust
    /// # use egui::{Button, IntoAtoms, __run_test_ui};
    /// # __run_test_ui(|ui| {
    /// let selected = true;
    /// ui.add(Button::new("toggle me").selected(selected).frame_when_inactive(!selected).frame(true));
    /// # });
    /// ```
    ///
    /// See also:
    ///   - [`Ui::selectable_value`]
    ///   - [`Ui::selectable_label`]
    pub fn selectable(selected: bool, atoms: impl IntoAtoms<'a>) -> Self {
        Self::new(atoms)
            .selected(selected)
            .frame_when_inactive(selected)
            .frame(true)
    }

    /// Set the wrap mode for the text.
    ///
    /// By default, [`crate::Ui::wrap_mode`] will be used, which can be overridden with [`crate::Style::wrap_mode`].
    ///
    /// Note that any `\n` in the text will always produce a new line.
    #[inline]
    pub fn wrap_mode(mut self, wrap_mode: TextWrapMode) -> Self {
        self.layout = self.layout.wrap_mode(wrap_mode);
        self
    }

    /// Set [`Self::wrap_mode`] to [`TextWrapMode::Wrap`].
    #[inline]
    pub fn wrap(self) -> Self {
        self.wrap_mode(TextWrapMode::Wrap)
    }

    /// Set [`Self::wrap_mode`] to [`TextWrapMode::Truncate`].
    #[inline]
    pub fn truncate(self) -> Self {
        self.wrap_mode(TextWrapMode::Truncate)
    }

    /// Override background fill color. Note that this will override any on-hover effects.
    /// Calling this will also turn on the frame.
    #[inline]
    pub fn fill(mut self, fill: impl Into<Color32>) -> Self {
        self.fill = Some(fill.into());
        self
    }

    /// Override button stroke. Note that this will override any on-hover effects.
    /// Calling this will also turn on the frame.
    #[inline]
    pub fn stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke = Some(stroke.into());
        self.frame = Some(true);
        self
    }

    /// Turn off the frame
    #[inline]
    pub fn frame(mut self, frame: bool) -> Self {
        self.frame = Some(frame);
        self
    }

    /// If `false`, the button will not have a frame when inactive.
    ///
    /// Default: `true`.
    ///
    /// Note: When [`Self::frame`] (or `ui.visuals().button_frame`) is `false`, this setting
    /// has no effect.
    #[inline]
    pub fn frame_when_inactive(mut self, frame_when_inactive: bool) -> Self {
        self.frame_when_inactive = frame_when_inactive;
        self
    }

    /// By default, buttons senses clicks.
    /// Change this to a drag-button with `Sense::drag()`.
    #[inline]
    pub fn sense(mut self, sense: Sense) -> Self {
        self.layout = self.layout.sense(sense);
        self
    }

    /// Set the minimum size of the button.
    #[inline]
    pub fn min_size(mut self, min_size: Vec2) -> Self {
        self.min_size = min_size;
        self
    }

    /// Set the rounding of the button.
    #[inline]
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = Some(corner_radius.into());
        self
    }

    #[inline]
    #[deprecated = "Renamed to `corner_radius`"]
    pub fn rounding(self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius(corner_radius)
    }

    /// Show some text on the right side of the button, in weak color.
    ///
    /// Designed for menu buttons, for setting a keyboard shortcut text (e.g. `Ctrl+S`).
    ///
    /// The text can be created with [`crate::Context::format_shortcut`].
    ///
    /// See also [`Self::right_text`].
    #[inline]
    pub fn shortcut_text(mut self, shortcut_text: impl Into<Atom<'a>>) -> Self {
        let mut atom = shortcut_text.into();
        atom.kind = match atom.kind {
            AtomKind::Text(text) => AtomKind::Text(text.weak()),
            other => other,
        };
        self.layout.push_right(Atom::grow());
        self.layout.push_right(atom);
        self
    }

    /// Show some text on the right side of the button.
    #[inline]
    pub fn right_text(mut self, right_text: impl Into<Atom<'a>>) -> Self {
        self.layout.push_right(Atom::grow());
        self.layout.push_right(right_text.into());
        self
    }

    /// If `true`, mark this button as "selected".
    #[inline]
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Set the color of the underline.
    #[inline]
    pub fn underline_color(mut self, underline_color: impl Into<Color32>) -> Self {
        self.underline_color = Some(underline_color.into());
        self
    }

    /// Set the inset of the button.
    #[inline]
    pub fn inset(mut self, inset: impl Into<Vec2>) -> Self {
        self.inset = inset.into();
        if self.hover_inset == Vec2::splat(-2.0) {
            // If hover_inset is not set, use the same inset for hover
            self.hover_inset = self.inset;
        }
        self
    }

    /// Set the inset of the button when hovered.
    #[inline]
    pub fn hover_inset(mut self, hover_inset: impl Into<Vec2>) -> Self {
        self.hover_inset = hover_inset.into();
        self
    }

    /// Show the button and return a [`AtomLayoutResponse`] for painting custom contents.
    pub fn atom_ui(self, ui: &mut Ui) -> AtomLayoutResponse {
        let ButtonWithUnderline {
            mut layout,
            fill,
            stroke,
            frame,
            frame_when_inactive,
            mut min_size,
            corner_radius,
            selected,
            underline_color,
            color,
            inset,
            hover_inset,
        } = self;

        let text = layout.text().map(String::from);

        let has_frame_margin = frame.unwrap_or_else(|| ui.visuals().button_frame);

        let mut button_padding = if has_frame_margin {
            ui.spacing().button_padding
        } else {
            Vec2::ZERO
        };

        let mut prepared = layout
            .frame(Frame::new().inner_margin(button_padding))
            .min_size(min_size)
            .allocate(ui);

        let focus = prepared.response.hovered() || prepared.response.is_pointer_button_down_on() || prepared.response.has_focus();

        let mut inner_margin;
        let response = if ui.is_rect_visible(prepared.response.rect) {
            let visuals = ui.style().interact_selectable(&prepared.response, selected);

            let visible_frame = if frame_when_inactive {
                has_frame_margin
            } else {
                has_frame_margin
                    && focus
            };

            prepared.fallback_text_color = visuals.text_color();

            if visible_frame {
                let stroke = stroke.unwrap_or(visuals.bg_stroke);
                let fill = fill.unwrap_or(visuals.weak_bg_fill);
                prepared.frame = prepared
                    .frame
                    .inner_margin(
                        button_padding + Vec2::splat(visuals.expansion) - Vec2::splat(stroke.width),
                    )
                    .outer_margin(-Vec2::splat(visuals.expansion))
                    .fill(fill)
                    .stroke(stroke)
                    .corner_radius(corner_radius.unwrap_or(visuals.corner_radius));
            };
            inner_margin = prepared.frame.inner_margin.clone();
            prepared.paint(ui)
        } else {
            inner_margin = Margin::default();
            AtomLayoutResponse::empty(prepared.response)
        };
        
        paint_underline(ui, &response.response, inner_margin, underline_color, if focus {
            hover_inset
        } else {
            inset
        });

        response.response.widget_info(|| {
            if let Some(text) = &text {
                WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), text)
            } else {
                WidgetInfo::new(WidgetType::Button)
            }
        });

        response
    }
}

fn paint_underline(
    ui: &mut Ui,
    response: &Response,
    margins: Margin,
    underline_color: Option<Color32>,
    inset: Vec2,
) {
    if let Some(underline_color) = underline_color {
        let rect = response.rect;
        let stroke = Stroke::new(1.0, underline_color);
        ui.painter().line_segment(
            [rect.left_bottom() + Vec2::new((margins.left as f32) + inset.x, 0.0), rect.right_bottom() + Vec2::new(-(margins.right as f32 + inset.y), 0.0)],
            stroke,
        );
    } else {
        let visuals = ui.visuals();
        let color = visuals.text_color();
        let rect = response.rect;
        let stroke = Stroke::new(1.0, color);
        ui.painter().line_segment(
            [rect.left_bottom() + Vec2::new((margins.left as f32) + inset.x, 0.0), rect.right_bottom() + Vec2::new(-(margins.right as f32 + inset.y), 0.0)],
            stroke,
        );
    }
}

impl Widget for ButtonWithUnderline<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.atom_ui(ui).response
    }
}

pub fn skill_frameplate(ui: &mut Ui, skill: &str, color: Color32, text_color: Color32, gradient: bool) -> () {
    let frame = Frame::new();
    // Make the frame's stroke a stronger version of the color given
    let stroke = Stroke::new(2.0, color.blend(Color32::from_black_alpha(100)));
    frame
        .fill(color)
        .inner_margin(2.0)
        .outer_margin(0.0)
        .corner_radius(CornerRadius::same(1))
        .stroke(stroke)
        .show(ui, |ui| {
        ui.label(egui::RichText::new(skill).color(text_color))
    });
}

pub fn socials(ui: &mut Ui, display: &str, link: &str, icon: &Option<String>) {
    let frame = Frame::new();
    let response = ui.scope_builder(
    UiBuilder::new()
        .sense(Sense::click()),
    |ui| {
            let mut frame_ui = frame
                .inner_margin(2.0)
                .outer_margin(0.0)
                .corner_radius(CornerRadius::same(1))
                .begin(ui);
            {
                frame_ui.content_ui.horizontal(|ui| {
                    if let Some(icon) = icon {
                        let image = Image::new(icon).fit_to_exact_size(Vec2::new(16.0, 16.0));
                        ui.add(image);
                    }
                    let base_text_color = ui.visuals().text_color();
                    ui.style_mut().interaction.selectable_labels = false;
                    ui.label(egui::RichText::new(display).color(base_text_color.blend(Color32::from_rgb(base_text_color.r(), base_text_color.g(), 255))));
                });
            }
            let response = frame_ui.allocate_space(ui);
            if response.hovered() {
                frame_ui.frame.fill = ui.visuals().noninteractive().bg_stroke.color;;
            }
            frame_ui.paint(ui);
            response
        },
    );
    if response.response.clicked() {
        if let Ok(_) = Url::new(link) { // Verifies valid link parsing
            if let Some(window) = window() {
                // Uses the link directly anyway since its been validated
                let _ = window.open_with_url_and_target(link, "_blank");
            }
        } else {
            log::debug!("Invalid URL: {}", link);
        }
    }
    if response.response.hovered() {
        ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
    }
}