use crate::styles::*;
use std::path::Path;

use fltk::{prelude::*,
    enums,
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


pub fn create_button(label: &str, size:(i32,i32)) -> button::Button {
        let mut button = button::Button::new(0,0,size.0,size.1,"").with_label(label);
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
        button.set_frame(FrameType::RFlatBox);
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


// Buggy, portare fuori indicazione del valore e magari metterlo in un output
pub fn create_slider(range: (f64, f64), step: (f64, i32)) -> fltk::valuator::Slider {
    let mut slider = fltk::valuator::Slider::new(0,0,200,30,"");
    slider.set_label("Window size: 1");
    slider.set_type(fltk::valuator::SliderType::Horizontal);
    slider.set_frame(fltk::enums::FrameType::FlatBox);
    // slider.set_slider_size(0.1);
    slider.set_range(range.0,range.1);
    slider.clear_visible_focus();
    slider.set_step(step.0,step.1);
    slider.set_color(enums::Color::from_hex(0x000050)); // Background of the bar
    slider.set_label_color(enums::Color::from_hex(THEME.get_text().color)); // Triangles on the extreme boxes
    slider.set_selection_color(enums::Color::from_hex(THEME.get_text().color)); // Bar and extreme boxes
    slider.set_align(enums::Align::Top);
    slider.handle(|t, event| {
                println!("{:?},  {:?}", event, event.contains(Event::Released));
                if event==Event::Released {
                    t.set_label(&format!("Window size: {}", 2.0*t.value()+1.0));
                    true
                } else {false}
            });
    slider
}


pub fn create_counter(box_size: (i32,i32), range: (f64, f64), step: f64, label: &str) -> (group::Pack, fltk::frame::Frame)  {
    let mut box_i = fltk::group::Pack::default().with_size(box_size.0, box_size.1).with_type(group::PackType::Vertical);
    box_i.end();
    box_i.set_label(label);
    box_i.set_align(Align::TopLeft);
    box_i.set_label_size(15);
    box_i.set_label_color(enums::Color::from_hex(THEME.get().label));

    let mut gr = fltk::group::Pack::default().with_size(box_size.0, box_size.1).with_type(group::PackType::Horizontal);
    box_i.add(&gr);
    gr.end();

    let mut out = fltk::frame::Frame::default()
        .center_of_parent()
        .with_size(50, 50)
        .with_label(&range.0.to_string());
    to_default_style(&mut out, &THEME);

    let mut up = create_button("+", (50,50));
    let mut down = create_button("-", (50,50));

    gr.add(&down);
    gr.add(&out);
    gr.add(&up);

    down.set_callback(move |p| {
                                let mut fr = p.parent().unwrap().child(1).unwrap();
                                let c = fr.label().parse::<f64>().unwrap() - step.clone();
                                if c >= range.0.clone() {
                                    fr.set_label(&(c).to_string());
                                }
                                });


    up.set_callback(move |p| {
                                let mut fr = p.parent().unwrap().child(1).unwrap();
                                let c = fr.label().parse::<f64>().unwrap() + step.clone();
                                if c <= range.1.clone() {
                                    fr.set_label(&(c).to_string());
                                }
                                });

    (box_i, out)

}


pub struct MyFrame {
    pub frame: fltk::frame::Frame,
    pub buffer: Vec<u8>,
    pub size: [i32;2],
}

impl MyFrame {
    pub fn new(size: [i32;2]) -> MyFrame {
        let frame = fltk::frame::Frame::default().with_size(size[0], size[1]);
        let mut buf: Vec<u8> = Vec::<u8>::with_capacity((size[0]*size[1]*3) as usize);
        unsafe { buf.set_len((size[0]*size[1]*3) as usize); buf.set_len((size[0]*size[1]*3) as usize);}
        MyFrame {
            frame,
            buffer: buf,
            size,
        }
    }

    pub fn update_frame(&mut self) {
        let image = fltk::image::RgbImage::new(&self.buffer, self.size[0], self.size[1], fltk::enums::ColorDepth::Rgb8);
        if let Ok(mut image) = image {
            image.scale(self.size[0],self.size[1],true,true);
            self.frame.set_image_scaled(Some(image));
        } else {
            println!("{:?}", image);
        }
    }

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
