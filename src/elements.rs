use std::sync::Arc;

use egui::{text_selection::visuals, Atom, AtomKind, AtomLayout, AtomLayoutResponse, Color32, CornerRadius, Frame, Image, IntoAtoms, Response, Sense, Stroke, TextWrapMode, TextureHandle, Ui, Vec2, Widget, WidgetInfo, WidgetText, WidgetType};

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
    image_tint_follows_text_color: bool,
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
            image_tint_follows_text_color: false,
            color: Color32::from_black_alpha(0),
            underline_color: None,
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

    /// If true, the tint of the image is multiplied by the widget text color.
    ///
    /// This makes sense for images that are white, that should have the same color as the text color.
    /// This will also make the icon color depend on hover state.
    ///
    /// Default: `false`.
    #[inline]
    pub fn image_tint_follows_text_color(mut self, image_tint_follows_text_color: bool) -> Self {
        self.image_tint_follows_text_color = image_tint_follows_text_color;
        self
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
            image_tint_follows_text_color,
            underline_color,
            color
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

        let response = if ui.is_rect_visible(prepared.response.rect) {
            let visuals = ui.style().interact_selectable(&prepared.response, selected);

            let visible_frame = if frame_when_inactive {
                has_frame_margin
            } else {
                has_frame_margin
                    && (prepared.response.hovered()
                        || prepared.response.is_pointer_button_down_on()
                        || prepared.response.has_focus())
            };

            if image_tint_follows_text_color {
                prepared.map_images(|image| image.tint(visuals.text_color()));
            }

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

            prepared.paint(ui)
        } else {
            AtomLayoutResponse::empty(prepared.response)
        };

        // (&self).paint_underline(ui, &response.response, &prepared.frame);

        response.response.widget_info(|| {
            if let Some(text) = &text {
                WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), text)
            } else {
                WidgetInfo::new(WidgetType::Button)
            }
        });

        response
    }

    // fn paint_underline(
    //     &self,
    //     ui: &mut Ui,
    //     response: &Response,
    //     frame: &Frame,
    // ) {
    //     if let Some(underline_color) = self.underline_color {
    //         let rect = response.rect;
    //         let stroke = Stroke::new(1.0, underline_color);
    //         ui.painter().line_segment(
    //             [rect.left_bottom() + Vec2::new(frame.inner_margin.left as f32, 0.0), rect.right_bottom() + Vec2::new(-frame.inner_margin.right as f32, 0.0)],
    //             stroke,
    //         );
    //     } else {
    //         let visuals = ui.visuals();
    //         let color = visuals.text_color();
    //         let stroke = Stroke::new(1.0, color);
    //         ui.painter().line_segment(
    //             [response.rect.left_bottom(), response.rect.right_bottom()],
    //             stroke,
    //         );
    //     }
    // }
}

impl Widget for ButtonWithUnderline<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.atom_ui(ui).response
    }
}