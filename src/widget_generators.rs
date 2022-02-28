use crate::styles::*;
use std::path::Path;

use fltk::{prelude::*,
    enums::{FrameType, Event, LabelType},
    button,
    input,
    text};


#[derive(Clone, Debug, Copy)]
pub enum Mess {
    Prev,
    Next,
    Chrono,
    Search,
    Children,
}

pub fn create_button(label: &str, style: &Theme) -> button::Button {
        let mut button = button::Button::default().with_size(100,50).with_label(label);
        to_button_style(&mut button, style);
        button
}



pub fn create_image(path: &Path, size: [i32; 2]) -> fltk::frame::Frame {
    let mut frame = fltk::frame::Frame::default().with_size(size[0], size[1]);
    let image = fltk::image::PngImage::load(path);
    if let Ok(mut image) = image {
        image.scale(size[0],size[1],true,true);
        frame.set_image_scaled(Some(image));
    }
    frame

}

pub fn create_text_widget(text: &str, style: &Theme) -> text::TextDisplay {
    let mut buf = text::TextBuffer::default();
    buf.set_text(text);
    let mut txt = text::TextDisplay::default();
    txt.set_buffer(buf);
    txt.wrap_mode(text::WrapMode::AtBounds,200);
    to_default_style(&mut txt, style);
    to_text_style(&mut txt, style);
    txt.set_frame(FrameType::FlatBox);
    txt
}

pub fn create_output(text: &str, style: &Theme) -> fltk::output::MultilineOutput {
    let mut txt = fltk::output::MultilineOutput::default();
    to_default_style(&mut txt, style);
    to_output_style(&mut txt, style);
    txt.set_value(text);
    txt
}

// pub fn update_buf<T: DisplayExt>(text: &str, txt: &mut T) {
//     let mut buf = text::TextBuffer::default();
//     buf.set_text(text);
//     if txt.was_deleted() {
//         txt.set_buffer(buf);
//     }
// }

pub fn create_input(tooltip: &str, style: &Theme) -> input::Input {
    let mut input = input::Input::default();
    input.set_value(tooltip);
    input.set_label(tooltip);
    input.set_label_type(LabelType::None);
    input.handle(|i, event| {
        match event {
            Event::Push => {if i.value()==i.label() {i.set_value("")} true},
            Event::Leave => {if i.value()=="" {i.set_value(&i.label())}; true},
            _ => true
        }
        }
    );
    to_input_style(&mut input, style);
    input
}
