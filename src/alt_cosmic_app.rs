use cosmic::iced::{Alignment, Element, Length};
use cosmic::widget::{button, column, nav_bar, row, text};
use cosmic::{Application, ApplicationExt, Command};

fn main() -> cosmic::Result {
    App::run(cosmic::Settings::default())
}

// 1. The MESSAGE (Action)
#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementCount,
}

// 2. The MODEL (State)
// This is the "App Model" you were looking for.
struct App {
    count: u32,
}

impl Application for App {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = cosmic::Theme;

    // Initialize the Model
    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (App { count: 0 }, Command::none())
    }

    // 3. The UPDATE (Logic)
    // This transitions the Model from one state to the next based on a Message.
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::IncrementCount => {
                self.count += 1; // Mutating the state
                println!("Count is now: {}", self.count);
            }
        }
        Command::none()
    }

    // 4. The VIEW (UI)
    // A declarative representation of the current Model.
    fn view(&self) -> Element<Self::Message> {
        // The Side Navigation Bar
        let sidebar = nav_bar::nav_bar(
            column![
                text("Dashboard").size(20),
                button("Click to Increment").on_press(Message::IncrementCount)
            ]
            .padding(20)
            .spacing(15)
            .align_items(Alignment::Start),
        )
        .width(Length::Fixed(250.0));

        // Main Content Area showing the state
        let content = column![text(format!("Button pressed {} times", self.count)).size(30)]
            .width(Length::Fill)
            .align_items(Alignment::Center);

        // Combine into a horizontal layout
        row![sidebar, content].into()
    }
}
