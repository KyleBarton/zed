use gpui::{Entity, EventEmitter, ParentElement, StyledText};
use settings::Settings;
use theme::ThemeSettings;
use ui::{Color, IntoElement, Render, Styled, StyledExt, px};
use workspace::{ToolbarItemEvent, ToolbarItemView};

use crate::Editor;

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
// * [] have an optionl on toolbaritem to remove all the padding
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

        if let Some(outline_items) = editor.read(cx).sticky_headers(cx) {
            let mut div = gpui::div().v_flex().w_full();
            div.extend(outline_items.into_iter().map(|outline_item| {
                let mut text_style = window.text_style();
                let font = ThemeSettings::get_global(cx).buffer_font.clone();
                text_style.font_family = font.family.clone();
                text_style.font_features = font.features.clone();
                text_style.font_style = font.style;
                text_style.font_weight = font.weight;
                text_style.color = Color::Muted.color(cx);

                let text_content =
                    outline_item.range.start.row.to_string() + " " + &outline_item.text;
                gpui::div()
                    .h(px(20.))
                    .w_full()
                    .child(
                        StyledText::new(text_content)
                            .with_default_highlights(&text_style, outline_item.highlight_ranges),
                    )
                    .into_any_element()
            }));
            div.into_any_element()
        } else {
            gpui::Empty.into_any_element()
        }
    }
}
