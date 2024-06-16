use iced::{executor, window};
use iced::widget::{button, checkbox, text, Column};
use iced::{alignment, Application, Element, Theme, Alignment, Command, Length, Settings};

struct Sender {
    now_state: bool,
    upload_state: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Toggled(bool),
    Exit,
}

impl Application for Sender {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Sender, Command<Self::Message>) {
        (
            Sender {
                now_state: false,
                upload_state: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Now sender")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Toggled(value) => {
                self.now_state = value;
                Command::none()
            }
            Message::Exit => Command::from(window::close(window::Id::MAIN)),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let toggle = checkbox("checkbox", self.now_state)
            .on_toggle(Message::Toggled);
        let exit = button(
            text("Exit")
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .width(100)
        .padding(10)
        .on_press(Message::Exit);

        let content = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(toggle)
            .push(exit);

        Element::from(content)
    }
}

pub fn main() -> iced::Result {
    let settings: Settings<()> = iced::settings::Settings  {
        window: window::Settings {
            size: iced::Size::new(400.0, 300.0),
            resizable: true,
            decorations: true,
            position: window::Position::Centered,
            ..Default::default()
        },
        ..Default::default()
    };
    Sender::run(settings)
}
