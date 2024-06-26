use base64::{engine::general_purpose::STANDARD, Engine as _};
use iced::widget::{button, checkbox, combo_box, text, text_input, Column};
use iced::{
    alignment, executor, window, Alignment, Application, Command, Element, Length, Renderer,
    Settings, Theme,
};
use reqwest::{blocking::Client, header::CONTENT_TYPE, StatusCode};
use serde_json::to_string;
use std::{collections::HashMap, env, error::Error, fs};

#[derive(Clone, Debug)]
enum Role {
    Family,
    Friend,
    Random,
}

impl Role {
    const ALL: [Role; 3] = [Role::Family, Role::Friend, Role::Random];
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Role::Family => "family",
                Role::Friend => "friend",
                Role::Random => "random",
            }
        )
    }
}

fn read_image_file(location: &String) -> String {
    match fs::read(location) {
        Err(_) => return String::new(),
        Ok(file_data) => return STANDARD.encode(&file_data),
    }
}

fn post_data(sender: &Sender) -> Result<StatusCode, Box<dyn Error>> {
    let role: String = match sender.selected_role {
        Some(Role::Family) => "family".into(),
        Some(Role::Friend) => "friend".into(),
        Some(Role::Random) => "random".into(),
        None => panic!("Unprocessable role"),
    };

    let params = match sender.now_state {
        false => HashMap::from([
            ("role", role),
            ("session", sender.session.clone()),
            ("title", String::new()),
            ("text", String::new()),
            ("image", String::new()),
        ]),
        true => {
            let encoded_image =
                read_image_file(&sender.image.clone().trim_matches('"').to_string());
            HashMap::from([
                ("role", role),
                ("session", sender.session.clone()),
                ("title", sender.title.clone()),
                ("text", sender.text.clone()),
                ("image", encoded_image),
            ])
        }
    };

    let params = to_string(&params)?;

    let url = env::var("NOW_URL").expect("NO env named NOW_URL") + "/post";

    let client = Client::new();
    let response = client
        .post(url)
        .body(params)
        .header(CONTENT_TYPE, "application/json")
        .send()?;

    Ok(response.status())
}

struct Sender {
    now_state: bool,
    roles: combo_box::State<Role>,
    selected_role: Option<Role>,
    session: String,
    title: String,
    text: String,
    image: String,
    state: String,
}

#[derive(Debug, Clone)]
enum Message {
    NowToggled(bool),
    TextTitle(String),
    TextText(String),
    TextImage(String),
    RoleSelected(Role),
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
                now_state: true,
                roles: combo_box::State::new(Role::ALL.to_vec()),
                selected_role: Some(Role::Family),
                session: env::var("NOW_SESSION").expect("NO env named NOW_SESSION"),
                title: String::new(),
                text: String::new(),
                image: String::new(),
                state: String::from("Press Uplaod button"),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Now sender")
    }

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
            Message::RoleSelected(role) => {
                self.selected_role = Some(role);
                Command::none()
            }
            Message::Upload => {
                match post_data(&self) {
                    Ok(StatusCode::OK) => self.state = String::from("Ok"),
                    Ok(statuscode) => self.state = String::from(statuscode.as_str()),
                    Err(e) => self.state = format!("Error: {}", e),
                }
                Command::none()
            }
            Message::Exit => Command::from(window::close(window::Id::MAIN)),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let now_toggle = checkbox("Now Status", self.now_state).on_toggle(Message::NowToggled);

        let role_combo = combo_box(
            &self.roles,
            "Role",
            self.selected_role.as_ref(),
            Message::RoleSelected,
        );

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

        let result_text = text(self.state.clone())
            .width(300)
            .horizontal_alignment(alignment::Horizontal::Center);

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
            .push(role_combo)
            .push(title_input)
            .push(text_text_input)
            .push(image_input)
            .push(upload_button)
            .push(result_text)
            .push(exit);

        Element::from(content)
    }
}

pub fn main() -> iced::Result {
    let settings: Settings<()> = iced::settings::Settings {
        window: window::Settings {
            size: iced::Size::new(300.0, 425.0),
            resizable: false,
            decorations: true,
            position: window::Position::Centered,
            ..Default::default()
        },
        ..Default::default()
    };
    Sender::run(settings)
}
