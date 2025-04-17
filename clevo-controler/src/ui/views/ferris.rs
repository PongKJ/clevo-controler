use crate::ui::mainwindow::Tab;
use iced::{
    Alignment, Element, Length,
    advanced::image::Bytes,
    widget::{Column, Container, Slider, Text, image::Handle, image::Image},
};
use iced_aw::tab_bar::TabLabel;

#[derive(Debug, Clone)]
pub enum FerrisMessage {
    ImageWidthChanged(f32),
}

#[derive(Debug, Default)]
pub struct FerrisTab {
    ferris_width: f32,
}

impl FerrisTab {
    pub fn new() -> Self {
        FerrisTab {
            ferris_width: 100.0,
        }
    }
    pub fn update(&mut self, message: FerrisMessage) {
        match message {
            FerrisMessage::ImageWidthChanged(width) => {
                self.ferris_width = width;
            }
        }
    }
}

impl Tab for FerrisTab {
    type Message = FerrisMessage;
    fn title(&self) -> String {
        "Ferris".to_string()
    }
    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::Hear, ())
    }
}
