use std::{path::Path, sync::Arc, time::Duration};

use anyhow::anyhow;
use gpui::{
    black, div, img, prelude::*, pulsating_between, px, red, size, Animation, AnimationExt, App,
    Application, Asset, AssetLogger, AssetSource, Bounds, Context, Hsla, ImageAssetLoader,
    ImageCacheError, ImgResourceLoader, Length, Pixels, RenderImage, Resource, SharedString,
    Window, WindowBounds, WindowOptions, LOADING_DELAY,
};

struct Assets {}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> gpui::Result<Option<std::borrow::Cow<'static, [u8]>>> {
        std::fs::read(path)
            .map(Into::into)
            .map_err(Into::into)
            .map(Some)
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(std::fs::read_dir(path)?
            .filter_map(|entity| {
                Some(SharedString::from(
                    entity.ok()?.path().to_string_lossy().to_string(),
                ))
            })
            .collect::<Vec<_>>())
    }
}

const IMAGE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/image/app-icon.png");

#[derive(Clone, Copy, Hash)]
struct LoadImageParameters {
    timeout: Duration,
    fail: bool,
}

struct LoadImageWithParameters {}

impl Asset for LoadImageWithParameters {
    type Source = LoadImageParameters;

    type Output = Result<Arc<RenderImage>, ImageCacheError>;

    fn load(
        parameter: Self::Source,
        cx: &mut App,
    ) -> impl std::future::Future<Output = Self::Output> + Send + 'static {
        let timer = cx.background_executor().timer(parameter.timeout);
        let data = AssetLogger::<ImageAssetLoader>::load(
            Resource::Path(Path::new(IMAGE).to_path_buf().into()),
            cx,
        );
        async move {
            timer.await;
            if parameter.fail {
                log::error!("Intentionaly failed to load image");
                Err(anyhow!("Failed to load image").into())
            } else {
                data.await
            }
        }
    }
}

struct ImageLoadingExample {}

impl ImageLoadingExample {
    fn loading_element() -> impl IntoElement {
        div().size_full().flex_none().p_0p5().rounded_xs().child(
            div().size_full().with_animation(
                "loading-bg",
                Animation::new(Duration::from_secs(3))
                    .repeat()
                    .with_easing(pulsating_between(0.04, 0.24)),
                move |this, delta| this.bg(black().opacity(delta)),
            ),
        )
    }

    fn fallback_element() -> impl IntoElement {
        let fallback_color: Hsla = black().opacity(0.5);

        div().size_full().flex_none().p_0p5().child(
            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .rounded_xs()
                .text_sm()
                .text_color(fallback_color)
                .border_1()
                .border_color(fallback_color)
                .child("?"),
        )
    }
}

impl Render for ImageLoadingExample {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().flex().flex_col().size_full().justify_around().child(
            div().flex().flex_row().w_full().justify_around().child(
                div()
                    .flex()
                    .bg(gpui::white())
                    .size(Length::Definite(Pixels(300.0).into()))
                    .justify_center()
                    .items_center()
                    .child({
                        let image_source = LoadImageParameters {
                            timeout: LOADING_DELAY.saturating_sub(Duration::from_millis(25)),
                            fail: false,
                        };

                        img(move |window: &mut Window, cx: &mut App| {
                            window.use_asset::<LoadImageWithParameters>(&image_source, cx)
                        })
                        .id("image-1")
                        .border_1()
                        .size_12()
                        .with_fallback(|| Self::fallback_element().into_any_element())
                        .border_color(red())
                        .with_loading(|| Self::loading_element().into_any_element())
                        .on_click(move |_, _, cx| {
                            cx.remove_asset::<LoadImageWithParameters>(&image_source);
                        })
                    })
                    .child({
                        // load after a long deplay
                        let image_source = LoadImageParameters {
                            timeout: Duration::from_secs(5),
                            fail: false,
                        };

                        img(move |window: &mut Window, cx: &mut App| {
                            window.use_asset::<LoadImageWithParameters>(&image_source, cx)
                        })
                        .id("image-2")
                        .with_fallback(|| Self::fallback_element().into_any_element())
                        .with_loading(|| Self::loading_element().into_any_element())
                        .size_12()
                        .border_1()
                        .border_color(red())
                        .on_click(move |_, _, cx| {
                            cx.remove_asset::<LoadImageWithParameters>(&image_source);
                        })
                    })
                    .child({
                        // Fail to load image after a long delay
                        let image_source = LoadImageParameters {
                            timeout: Duration::from_secs(5),
                            fail: true,
                        };

                        // Fail to load after a long delay
                        img(move |window: &mut Window, cx: &mut App| {
                            window.use_asset::<LoadImageWithParameters>(&image_source, cx)
                        })
                        .id("image-3")
                        .with_fallback(|| Self::fallback_element().into_any_element())
                        .with_loading(|| Self::loading_element().into_any_element())
                        .size_12()
                        .border_1()
                        .border_color(red())
                        .on_click(move |_, _, cx| {
                            cx.remove_asset::<LoadImageWithParameters>(&image_source);
                        })
                    })
                    .child({
                        // Ensure that the normal image loader doesn't spam logs
                        let image_resource = Path::new(
                            "this/file/really/shouldn't/exist/or/won't/be/an/image/I/hope",
                        )
                        .to_path_buf();
                        img(image_resource.clone())
                            .id("image-1")
                            .border_1()
                            .size_12()
                            .with_fallback(|| Self::fallback_element().into_any_element())
                            .border_color(red())
                            .with_loading(|| Self::loading_element().into_any_element())
                            .on_click(move |_, _, cx| {
                                cx.remove_asset::<ImgResourceLoader>(
                                    &image_resource.clone().into(),
                                );
                            })
                    }),
            ),
        )
    }
}

fn main() {
    env_logger::init();
    Application::new()
        .with_assets(Assets {})
        .run(|cx: &mut App| {
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(300.), Pixels(300.)),
                    cx,
                ))),
                ..Default::default()
            };
            cx.open_window(options, |_, cx| {
                cx.activate(false);
                cx.new(|_| ImageLoadingExample {})
            })
            .unwrap();
        });
}
