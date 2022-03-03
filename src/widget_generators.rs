use crate::styles::*;
use std::path::Path;

use fltk::{prelude::*,
    enums::{FrameType, Event, LabelType, Align},
    group,
    button,
    input,
    text};

use super::THEME;

#[derive(Clone, Debug, Copy)]
pub enum Mess {
    Prev,
    Next,
    Chrono,
    Search,
    Children,
}


pub fn create_button(label: &str) -> button::Button {
        let mut button = button::Button::default().with_size(100,50).with_label(label);
        to_button_style(&mut button, &THEME);
        button
}

pub fn create_toggle_button(label: &str, box_size: (i32,i32)) -> (fltk::group::Group, button::ToggleButton) {
        let group = fltk::group::Group::default().with_size(box_size.0, box_size.1);
        create_text_widget(label).with_pos(0,box_size.1/2).with_size(box_size.0/2,box_size.1/2);
        let mut button = button::ToggleButton::default().with_size(box_size.0/4,20)
                            .with_pos(box_size.0*3/4,box_size.1/2)
                            .with_label("@+2circle")
                            .with_align(Align::Inside | Align::Left | Align::Wrap);
        group.end();
        to_button_style(&mut button, &THEME);
        button.set_selection_color(fltk::enums::Color::from_hex(0x000090));
        button.handle(|t, event| {
            if event==Event::Push {
                if t.is_toggled() {
                    t.set_align(Align::Inside | Align::Left | Align::Wrap);

                } else {
                    t.set_align(Align::Inside | Align::Right | Align::Wrap);
                }
            }
            true
        });

        (group, button)
}

pub fn create_image_from_file(path: &Path, size: [i32; 2]) -> fltk::frame::Frame {
    let mut frame = fltk::frame::Frame::default().with_size(size[0], size[1]);
    let image = fltk::image::PngImage::load(path);
    if let Ok(mut image) = image {
        image.scale(size[0],size[1],true,true);
        frame.set_image_scaled(Some(image));
    }
    frame

}

pub fn create_image_from_buffer(buffer: &[u8], size: [i32; 2]) -> fltk::frame::Frame {
    let mut frame = fltk::frame::Frame::default().with_size(size[0], size[1]);
    let image = fltk::image::RgbImage::new(buffer, size[0], size[1], fltk::enums::ColorDepth::Rgb8);
    if let Ok(mut image) = image {
        image.scale(size[0],size[1],true,true);
        frame.set_image_scaled(Some(image));
    } else {
        println!("{:?}", image);
    }
    frame

}

pub fn create_text_widget(text: &str) -> text::TextDisplay {
    let mut buf = text::TextBuffer::default();
    buf.set_text(text);
    let mut txt = text::TextDisplay::default();
    txt.set_buffer(buf);
    txt.wrap_mode(text::WrapMode::AtBounds,200);
    to_default_style(&mut txt, &THEME);
    to_text_style(&mut txt, &THEME);
    txt.set_frame(FrameType::FlatBox);
    txt
}



pub fn create_output(text: &str) -> fltk::output::MultilineOutput {
    let mut txt = fltk::output::MultilineOutput::default();
    to_default_style(&mut txt, &THEME);
    to_output_style(&mut txt, &THEME);
    txt.set_value(text);
    txt
}

// Wrappa un widget in un gruppo scroll
pub fn wrap_in_scroll<W: WidgetExt>(child: &W, pos_size: (i32,i32,i32,i32)) ->  group::Scroll {
    let mut scroll = group::Scroll::new(pos_size.0,pos_size.1,pos_size.2,pos_size.3,"").with_type(group::ScrollType::Vertical);
    scroll.end();
    to_scrollbar_style(&mut scroll.scrollbar(), &THEME);
    to_default_style(&mut scroll, &THEME);
    scroll.add(child);
    scroll
}

// pub fn update_buf<T: DisplayExt>(text: &str, txt: &mut T) {
//     let mut buf = text::TextBuffer::default();
//     buf.set_text(text);
//     if txt.was_deleted() {
//         txt.set_buffer(buf);
//     }
// }

pub fn create_input(tooltip: &str) -> input::Input {
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
    to_input_style(&mut input, &THEME);
    input
}
