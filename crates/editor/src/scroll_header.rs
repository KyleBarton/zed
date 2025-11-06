use gpui::{Entity, EventEmitter, HitboxBehavior, ParentElement, StyledText, bounds};
use settings::Settings;
use theme::ThemeSettings;
use ui::{Color, IntoElement, Render, Styled, StyledExt, px};
use workspace::{ToolbarItemEvent, ToolbarItemView};

use crate::{
    Editor,
    element::{gutter_bounds, max_line_number_width},
};

pub struct ScrollHeader {
    editor: Option<Entity<Editor>>,
}

impl ScrollHeader {
    pub fn new() -> Self {
        Self { editor: None }
    }
}

impl EventEmitter<ToolbarItemEvent> for ScrollHeader {}

impl ToolbarItemView for ScrollHeader {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn workspace::ItemHandle>,
        window: &mut ui::Window,
        cx: &mut ui::Context<Self>,
    ) -> workspace::ToolbarItemLocation {
        if let Some(pane_item) = active_pane_item
            && let Some(editor) = pane_item.act_as::<Editor>(cx)
            && editor.read(cx).buffer().read(cx).is_singleton()
        {
            self.editor = Some(editor);
            workspace::ToolbarItemLocation::Secondary
        } else {
            self.editor = None;
            workspace::ToolbarItemLocation::Hidden
        }
    }
}

// * [] try to re-use breadcrumbs code (but with the differennce...)
//   - Getting some items from it
//   - Probably need to improve the query of it as well
// * [] have an optional on toolbaritem to remove all the padding
// outline panel has code to syntax highlight fragments from the buffer

impl Render for ScrollHeader {
    fn render(
        &mut self,
        window: &mut ui::Window,
        cx: &mut ui::Context<Self>,
    ) -> impl ui::IntoElement {
        let Some(editor) = self.editor.as_ref() else {
            return gpui::Empty.into_any_element();
        };
        let widest_line_number = editor
            .read(cx)
            .buffer()
            .read(cx)
            .snapshot(cx)
            .widest_line_number();

        let snapshot = editor.update(cx, |editor, cx| editor.snapshot(window, cx));

        let rem_size = window.rem_size();
        let style = editor.read(cx).style.clone().unwrap_or_default();
        let font_id = window.text_system().resolve_font(&style.text.font());
        let font_size = style.text.font_size.to_pixels(rem_size);
        let max_line_width = max_line_number_width(widest_line_number, &style, window);

        let gutter_dimensions = snapshot
            .gutter_dimensions(font_id, font_size, max_line_width, cx)
            .unwrap_or_default();

        if let Some(outline_items) = editor.read(cx).sticky_headers(cx) {
            let mut outer_div = gpui::div().h_flex().w_full();

            let mut text_div = gpui::div()
                .left(gutter_dimensions.full_width())
                .v_flex()
                .w_full();

            text_div.extend(outline_items.iter().map(|outline_item| {
                let mut text_style = window.text_style();
                let font = ThemeSettings::get_global(cx).buffer_font.clone();
                text_style.font_family = font.family.clone();
                text_style.font_features = font.features.clone();
                text_style.font_style = font.style;
                text_style.font_weight = font.weight;
                text_style.color = Color::Muted.color(cx);

                gpui::div()
                    .h(text_style.line_height_in_pixels(window.rem_size()))
                    .w_full()
                    .child(
                        StyledText::new(outline_item.text.clone()).with_default_highlights(
                            &text_style,
                            outline_item.highlight_ranges.clone(),
                        ),
                    )
                    .justify_start()
                    .text_left()
                    .into_any_element()
            }));

            let mut gutter_div = gpui::div().v_flex().w(gutter_dimensions.full_width());

            gutter_div.extend(outline_items.into_iter().map(|outline_item| {
                let mut text_style = window.text_style();
                let font = ThemeSettings::get_global(cx).buffer_font.clone();
                text_style.font_family = font.family.clone();
                text_style.font_features = font.features.clone();
                text_style.font_style = font.style;
                text_style.font_weight = font.weight;
                text_style.color = Color::Muted.color(cx);

                gpui::div()
                    .h(text_style.line_height_in_pixels(window.rem_size()))
                    .w_full()
                    .child(
                        StyledText::new((outline_item.range.start.row + 1).to_string())
                            .with_default_highlights(&text_style, Vec::new()),
                    )
                    .text_center()
                    .into_any_element()
            }));

            outer_div.extend(vec![
                gutter_div.into_any_element(),
                text_div.into_any_element(),
            ]);

            outer_div.into_any_element()
        } else {
            gpui::Empty.into_any_element()
        }
    }
}
