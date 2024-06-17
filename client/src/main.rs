use iced::widget::{button, checkbox, text, text_input, Column};
use iced::{
    alignment, executor, window, Alignment, Application, Command, Element, Length, Renderer,
    Settings, Theme,
};
use reqwest::{Client, StatusCode};

const URL: &str = "http://127.0.0.1:8080";

// TODO: @random6 implement function
fn read_image_file(location: String) -> String {
    String::new()
}

// TODO: @random6 complete function
async fn post_data(sender: &Sender) -> StatusCode {
    let client = Client::new();
    let params = [
        ("title", sender.title.clone()),
        ("text", sender.text.clone()),
        ("image", sender.image.clone()),
    ];
    let response = client.post(URL).json(&params).send().await.unwrap();

    StatusCode::OK
}

struct Sender {
    now_state: bool,
    title: String,
    text: String,
    image: String,
}

#[derive(Debug, Clone)]
enum Message {
    NowToggled(bool),
    TextTitle(String),
    TextText(String),
    TextImage(String),
    Upload,
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
                title: String::new(),
                text: String::new(),
                image: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Now sender")
    }

    // TODO: @random6 repair condition
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::NowToggled(value) => {
                self.now_state = value;
                Command::none()
            }
            Message::TextTitle(title_input) => {
                self.title = title_input;
                Command::none()
            }
            Message::TextText(text_text_input) => {
                self.text = text_text_input;
                Command::none()
            }
            Message::TextImage(image_input) => {
                self.image = image_input;
                Command::none()
            }
            Message::Upload => {
                post_data(&self);
                Command::none()
            }
            Message::Exit => Command::from(window::close(window::Id::MAIN)),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let now_toggle = checkbox("Now Status", self.now_state).on_toggle(Message::NowToggled);

        let title_input: iced::widget::TextInput<'_, Message, Theme, Renderer> =
            text_input("Title", &self.title)
                .width(300)
                .on_input(|s| Message::TextTitle(s))
                .padding(10);

        let text_text_input: iced::widget::TextInput<'_, Message, Theme, Renderer> =
            text_input("Text", &self.text)
                .width(300)
                .on_input(|s| Message::TextText(s))
                .padding(10);

        let image_input: iced::widget::TextInput<'_, Message, Theme, Renderer> =
            text_input("Image", &self.image)
                .width(300)
                .on_input(|s| Message::TextImage(s))
                .padding(10);

        let upload_button = button(
            text("Upload")
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .width(100)
        .padding(10)
        .on_press(Message::Upload);

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
            .push(now_toggle)
            .push(title_input)
            .push(text_text_input)
            .push(image_input)
            .push(upload_button)
            .push(exit);

        Element::from(content)
    }
}

pub fn main() -> iced::Result {
    let settings: Settings<()> = iced::settings::Settings {
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
