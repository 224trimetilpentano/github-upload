use  plotters::style::{ShapeStyle, RGBAColor, RGBColor};
use  plotters::prelude::*;
use fltk::{prelude::*, enums, enums::{FrameType, Align}};

// Da mettere anche le scritte nei plots

pub fn to_default_style(widget: &mut impl WidgetExt, st: &Theme) {
    widget.set_color(enums::Color::from_hex(st.get().background));
    widget.set_selection_color(enums::Color::from_hex(st.get().background));
    widget.set_label_color(enums::Color::from_hex(st.get().label));
}

pub fn to_search_pack_style(widget: &mut impl WidgetExt, st: &Theme) {
    widget.set_color(enums::Color::from_hex(st.get().background));
    widget.set_label_color(enums::Color::from_hex(st.get().label));
    widget.set_align(Align::TopLeft);
    widget.set_label_size(20);
}


pub fn to_button_style(widget: &mut impl WidgetExt, st: &Theme) {
    widget.set_color(enums::Color::from_hex(0x000050));
    widget.set_selection_color(enums::Color::from_hex(0x000030));
    widget.set_label_color(enums::Color::from_hex(st.get().label));
    widget.clear_visible_focus();
    widget.set_frame(FrameType::FlatBox);
}

pub fn to_scrollbar_style(sc: &mut fltk::valuator::Scrollbar, st: &Theme) {
    sc.set_slider_frame(FrameType::FlatBox);
    sc.set_color(enums::Color::from_hex(0x000050)); // Background of the bar
    sc.set_label_color(enums::Color::from_hex(0x000050)); // Triangles on the extreme boxes
    sc.set_selection_color(enums::Color::from_hex(st.get_text().color)); // Bar and extreme boxes
}

pub fn to_input_style<T: WidgetExt + InputExt>(widget: &mut T , st: &Theme) {
    widget.set_color(enums::Color::from_hex(0x000050));
    widget.set_selection_color(enums::Color::from_hex(st.get_text().color));
    widget.set_label_color(enums::Color::from_hex(st.get().label));
    widget.set_frame(FrameType::FlatBox);
    widget.set_text_color(enums::Color::from_hex(st.get_text().color));
}

pub fn to_output_style<T: WidgetExt + InputExt>(widget: &mut T , st: &Theme) {
    widget.set_text_color(enums::Color::from_hex(st.get_text().color));
    widget.set_frame(FrameType::FlatBox);
}

pub fn to_text_style(widget: &mut impl DisplayExt, st: &Theme) {
    if widget.buffer().is_some() {
        widget.set_text_color(enums::Color::from_hex(st.get_text().color));
    }
    if widget.was_deleted() {
        widget.set_secondary_selection_color(enums::Color::from_hex(st.get().background));
    }
}

pub enum Theme {
    Dark,
    Light,
}

pub struct ThemeWid{
    pub background: u32,
    pub label: u32,
}

pub struct ThemeText {
    pub color: u32,
}

pub struct ThemeStr{
    pub background: RGBAColor,
    pub caption: RGBAColor,
    pub label: RGBAColor,
    pub histogram: ShapeStyle,
    pub tchart_line: ShapeStyle,
}

impl Theme {

    pub fn get_text(&self) -> ThemeText {
        match self {
            Theme::Dark => ThemeText {
                color:0xcccccc,
                },
            Theme::Light => ThemeText {
                color: 0x000066,
                },
        }
    }

    pub fn get(&self) -> ThemeWid {
        match self {
            Theme::Dark => ThemeWid {
                background:0x000030,
                label:0xcccccc,
                },
            Theme::Light => ThemeWid {
                background: 0xffffff,
                label: 0x000066,
                },
            }
    }

    pub fn get_plot(&self) -> ThemeStr {
        match self {
            Theme::Dark => ThemeStr {
                background:RGBColor(0,0,48).to_rgba(),
                caption: WHITE.to_rgba(),
                label: WHITE.to_rgba(),
                histogram: BLUE.mix(0.7).filled(),
                tchart_line: ShapeStyle{
                    color:  RED.to_rgba(),
                    filled: true,
                    stroke_width: 3,
                },
            },
            Theme::Light => ThemeStr {
                background: RGBColor(255,255,255).to_rgba(),
                caption: BLACK.to_rgba(),
                label: BLACK.to_rgba(),
                histogram: BLUE.mix(0.7).filled(),
                tchart_line: ShapeStyle{
                    color:  RED.to_rgba(),
                    filled: true,
                    stroke_width: 3,
                },
            }
        }
    }
}
