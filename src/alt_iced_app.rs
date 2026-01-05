use iced::widget::{button, column, text};
use iced::{Application, Command, Element, Settings, Theme, executor};

// The main entry point of the application
pub fn main() -> iced::Result {
    App::run(Settings::default())
}

// Represents the state of the application
#[derive(Default)]
struct App {
    message: String,
}

// Defines the messages that can be produced by user interactions
#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
}

// Implements the Application trait for the App struct
impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    // Initializes the application state
    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                message: String::from("Press the button"),
            },
            Command::none(),
        )
    }

    // Handles application logic when a message is received
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ButtonPressed => {
                self.message = String::from("Button Pressed!");
            }
        }
        Command::none()
    }

    // Renders the user interface
    fn view(&self) -> Element<Self::Message> {
        column![
            text(&self.message).size(30),
            button("Click Me!").on_press(Message::ButtonPressed),
        ]
        .spacing(20)
        .padding(20)
        .into()
    }

    // Sets the application title
    fn title(&self) -> String {
        String::from("Minimal Iced App")
    }
}
