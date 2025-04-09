mod button;
mod consts;
mod display;
mod logic;
mod root;
mod styles;

#[cfg(test)]
mod logic_test;

use gpui::*;
use root::*;

actions!(calculator, [Quit]);

fn main() {
    // Evetything in GPUI starts with an Application. You can create one with Application::new(), and kick off your application by passing a callback to Application::run(). Inside this callback, you can create a new window with App::open_window. and register first root view.
    Application::new().run(|cx| {
        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        cx.set_menus(vec![Menu {
            name: "Calculator".into(),
            items: vec![MenuItem::action("Quit", Quit)],
        }]);
        let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);

        let _ = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|cx| Root::new(cx)),
        );
    });
}

// GPUI offers three different registers depending on your need:
//// State management and communication with Entity's. Whenever you need to store application state that communicates between diefferent parts of your application, you will want to use GPUI's entities. Entities are owned by GPUI and are only accessible through an owned smart pointer similar to an Rc. See the app::Context moule for more information
//// High level, declarative UI with views. All UI in GPUI starts with a view. A view is simply an Entity that can be readered by implementing the Render trait. Views build a tree of elements, lay them out and style them with a tailwing-style API, and then give them to GPUI to turn into pixels. See the div element for all purpose swiss-army knife of rendering
//// low level, impretive UI with elements. Elements are the building blocks of UI in GPUI, and they provide a nice wrapper around an imperative API that provides as much flexibility and control as you need. Elements have total control over how they and their child elements are rendered and can be used for making efficient views into large lists. implement custom layouting for a code editor, and anything else you can think of. See the element module for more information
