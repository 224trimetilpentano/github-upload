use  plotters::style::{ShapeStyle, RGBAColor, RGBColor};
use  plotters::prelude::*;

// Da mettere anche le scritte nei plots


pub enum theme {
    Dark,
    Light,
}

pub struct theme_str{
    pub background: RGBAColor,
    pub caption: RGBAColor,
    pub label: RGBAColor,
    pub histogram: ShapeStyle,
    pub tchart_line: ShapeStyle,
}

impl theme {

    pub fn get(&self) -> theme_str {
        match self {
            theme::Dark => theme_str {
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
            theme::Light => theme_str {
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
