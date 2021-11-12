use  plotters::style::{ShapeStyle, RGBAColor, RGBColor};
use  plotters::prelude::*;

// Da mettere anche le scritte nei plots


pub enum Theme {
    Dark,
    Light,
}

pub struct ThemeStr{
    pub background: RGBAColor,
    pub caption: RGBAColor,
    pub label: RGBAColor,
    pub histogram: ShapeStyle,
    pub tchart_line: ShapeStyle,
}

impl Theme {

    pub fn get(&self) -> ThemeStr {
        match self {
            Theme::Dark => ThemeStr {
                background: RGBColor(59, 67, 74).to_rgba(),
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
                background: WHITE.to_rgba(),
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
