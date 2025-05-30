use gpui::{
    App, Application, Bounds, DisplayId, Hsla, Pixels, SharedString, Size, Window, WindowBackgroundAppearance, WindowBounds, WindowKind, WindowOptions, div, point, prelude::*, px, rgb
};

struct WindowContent {
    text: SharedString,
    bounds: Bounds<Pixels>,
    bg: Hsla,
}

impl Render for WindowContent {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let window_bounds = window.bounds();

        div()
            .flex()
            .flex_col()
            .bg(self.bg)
            .size_full()
            .items_center()
            .text_color(rgb(0xffffff))
            .child(self.text.clone())
            .child(
                div()
                    .flex()
                    .text_sm()
                    .items_center()
                    .size_full()
                    .child(format!(
                        "origin: {}, {} size: {}, {}",
                        self.bounds.origin.x,
                        self.bounds.origin.y,
                        self.bounds.size.width,
                        self.bounds.size.height,
                    ))
                    .child(format!(
                        "cx.bounds() origin: {}, {} size: {}, {}",
                        window_bounds.origin.x,
                        window_bounds.origin.y,
                        window_bounds.size.width,
                        window_bounds.size.height,
                    ))
            )
    }
}

fn build_window_options(display_id: DisplayId, bounds: Bounds<Pixels>) -> WindowOptions {
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        display_id: Some(display_id),
        titlebar: None,
        window_background: WindowBackgroundAppearance::Transparent,
        focus: false,
        show: true,
        kind: WindowKind::PopUp,
        is_movable: false,
        app_id: None,
        window_min_size: None,
        window_decorations: None,
    }            
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let size = Size {
            width: px(350.),
            height: px(75.),
        };
        let margin_offset = px(150.);
    
        for screen in cx.displays() {
            let bounds = Bounds {
                origin: point(margin_offset, margin_offset),
                size,
            };
    
            cx.open_window(
                build_window_options(screen.id(), bounds), |_, cx| {
                    cx.new(|_| WindowContent {
                        text: format!("Top Left {:?}", screen.id()).into(),
                        bg: gpui::red(),
                        bounds,
                    })
            })
            .unwrap();

            let bounds = Bounds {
                origin: screen.bounds().top_right() - point(size.width + margin_offset, - margin_offset),
                size,
            };

            cx.open_window(build_window_options(screen.id(), bounds), |_, cx: &mut App| {
                cx.new(|_| WindowContent {
                    text: format!("Top Right {:?}", screen.id()).into(),
                    bg: gpui::red(),
                    bounds,
                })
            })
            .unwrap();

            let bounds = Bounds {
                origin: screen.bounds().bottom_left() - point(- margin_offset, size.height + margin_offset),
                size,
            };
            cx.open_window(build_window_options(screen.id(), bounds), |_, cx: &mut App| {
                cx.new(|_| WindowContent {
                    text: format!("Bottom Left {:?}", screen.id()).into(),
                    bg: gpui::blue(),
                    bounds,
                })
            })
            .unwrap();

            let bounds = Bounds {
                origin: screen.bounds().bottom_right() - point(size.width + margin_offset, size.height + margin_offset),
                size,
            };
            cx.open_window(build_window_options(screen.id(), bounds), |_, cx: &mut App| {
                cx.new(|_| WindowContent {
                    text: format!("Bottom Right {:?}", screen.id()).into(),
                    bg: gpui::blue(),
                    bounds,
                })
            })
           .unwrap();

            let bounds = Bounds {
                origin: point(screen.bounds().center().x - size.center().x, margin_offset),
                size,
            };

            cx.open_window(build_window_options(screen.id(), bounds), |_, cx| {
                cx.new(|_| WindowContent {
                    text: format!("Top Center {:?}", screen.id()).into(),
                    bg: gpui::black(),
                    bounds,
                })
            })
            .unwrap();

            let bounds = Bounds {
                origin: point(margin_offset, screen.bounds().center().y - size.center().y),
                size,
            };

            cx.open_window(build_window_options(screen.id(), bounds), |_, cx| {
                cx.new(|_| WindowContent {
                    text: format!("Left Center {:?}", screen.id()).into(),
                    bg: gpui::black(),
                    bounds,
                })
            })
            .unwrap();
            
            let bounds = Bounds {
                origin: point(
                    screen.bounds().size.width - size.width - margin_offset,
                    screen.bounds().center().y - size.center().y,

                ),
                size,
            };

            cx.open_window(build_window_options(screen.id(), bounds), |_, cx| {
                cx.new(|_| WindowContent {
                    text: format!("Right Center {:?}", screen.id()).into(),
                    bg: gpui::black(),
                    bounds,
                })
            })
            .unwrap();

            let bounds = Bounds {
                origin: point(
                    screen.bounds().center().x - size.center().x,
                    screen.bounds().size.height - size.height - margin_offset,
                ),
                size,
            };
            cx.open_window(build_window_options(screen.id(), bounds), |_, cx| {
                cx.new(|_| WindowContent {
                    text: format!("Bottom Center {:?}", screen.id()).into(),
                    bg: gpui::black(),
                    bounds,
                })
            })
           .unwrap();
        }
    });
}