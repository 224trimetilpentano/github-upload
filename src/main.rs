// #![windows_subsystem = "windows"]
static THEME: Theme = Theme::Dark;

mod recs;
mod engine;
mod day_report;
mod tag_search;
mod week_widgets;
mod search_widgets;
mod styles;
mod widget_generators;

use week_widgets::*;
use search_widgets::*;
use styles::*;
use widget_generators::Mess;
use recs::rec_folder;

use fltk::{app, prelude::*,
    group,
    // button,
    enums::{Color, FrameType},
    window};


fn main() {
    let a = app::App::default();
    let window_icon = fltk::image::PngImage::load(rec_folder().join("hashtag_icon.png"));
    let mut wind = window::Window::new(0, 0, 1500, 775, "Rec");
    wind.set_icon(window_icon.ok());
    // Unico canale, ogni funzione delle tabs generano i widget e li passano in una closure,
    // insieme ai callbacks, poi le closure vengono chiamate nel loop
    // I messaggi sono definiti in widget_generators.rs
    let (s, r) = app::channel::<Mess>();

    wind.set_color(Color::from_hex(0x000060));
    let mut tabs = group::Tabs::new(0,20,wind.width(),880,"");
    to_button_style(&mut tabs, &THEME);

    let mut week_tab = group::Group::new(0,50,wind.width(),850,"Week\t\t");
    to_default_style(&mut week_tab, &THEME);
    week_tab.set_frame(FrameType::FlatBox);
    let mut week_clos = week_lay(&THEME, &s);
    week_tab.end();

    let mut search_tab = group::Group::new(0,50,wind.width(),850,"Search\t\t");
    to_default_style(&mut search_tab, &THEME);
    search_tab.set_frame(FrameType::FlatBox);
    let mut search_clos = search_lay(&THEME, &s);
    search_tab.end();

    tabs.end();
    wind.make_resizable(true);
    wind.end();
    wind.show();

    while a.wait() {
        if let Some(msg) = r.recv() {
            week_clos(msg);
            search_clos(msg);
        }
    }


}
