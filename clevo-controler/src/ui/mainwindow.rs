use iced::{
    Element, Font, Length,
    alignment::{Horizontal, Vertical},
    widget::{Column, Container, Text},
};
use iced_aw::{TabLabel, Tabs};

use crate::ui::views::ferris::{FerrisMessage, FerrisTab};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
enum TabId {
    #[default]
    Ferris,
}

#[derive(Debug)]
struct TabBar {
    active_tab: TabId,
    ferris_tab: FerrisTab,
}

#[derive(Clone, Debug)]
enum Message {
    TabSelected(TabId),
    Ferris(FerrisMessage),
}

pub trait Tab {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(Text::new(self.title()))
            .push(self.content())
            .align_x(iced::Alignment::Center);

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(20)
            .into()
    }

    fn content(&self) -> Element<'_, Self::Message>;
}
