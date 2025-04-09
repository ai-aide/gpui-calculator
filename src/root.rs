use gpui::AppContext;
use gpui::{
    div, rgb, Context, DefiniteLength, FocusHandle, InteractiveElement, IntoElement, KeyDownEvent,
    ParentElement, Render, Styled, Window,
};

use crate::button::*;
use crate::consts::*;
use crate::display::*;
use crate::logic::*;
use crate::styles::*;

pub struct Root {
    pub logic: Logic,
    focus_handle: FocusHandle,
}

impl Root {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let logic = Logic::new();

        Self {
            logic,
            focus_handle: cx.focus_handle(),
        }
    }

    fn get_buttons(&self, cx: &mut Context<Self>) -> Vec<Button> {
        let mut buttons = Vec::new();

        for button_type in BUTTONS {
            let basis = match button_type {
                ButtonType::Equal => 0.47,
                _ => 0.225,
            };

            let variant = match button_type {
                ButtonType::Equal => ButtonVariant::Primary,
                ButtonType::Number(_) => ButtonVariant::Neutral,
                ButtonType::Comma => ButtonVariant::Neutral,
                _ => ButtonVariant::Secondary,
            };

            let button = Button::new(button_type, basis, variant).on_click(cx.listener(
                move |this, _, _window, app| {
                    this.logic.on_button_pressed(button_type);
                    app.notify()
                },
            ));

            buttons.push(button);
        }

        buttons
    }
}

impl Render for Root {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let display_value = self.logic.get_display_value();
        let buttons = self.get_buttons(cx);

        let target = cx.new(|_cx| Display::new(display_value));

        div()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(
                |this, event: &KeyDownEvent, _: &mut Window, cx: &mut Context<Self>| {
                    this.logic.handle_key_input(&event.keystroke.key.as_str());
                    cx.notify();
                },
            ))
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(PAD_COLOR))
            .text_lg()
            .child(target)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .justify_center()
                    .h(DefiniteLength::Fraction(0.80))
                    .py(DefiniteLength::Fraction(0.02))
                    .gap(DefiniteLength::Fraction(0.02))
                    .children(buttons),
            )
    }
}
