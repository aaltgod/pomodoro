use iced::{
    alignment, button, executor, time, window, Alignment, Application, Button, Column,
    Command, Container, Element, Length, Row, Settings, Subscription, Text,
};
use std::time::{Duration, Instant};

pub fn main() -> iced::Result {
    Pomodoro::run(Settings {
        window: window::Settings {
            size: (style::WINDOW_WIDTH, style::WINDOW_HEIGHT), 
            position: window::Position::Centered,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

const FORTY_MINS_IN_SECS: u64 = 2400;

struct Pomodoro {
    count_down: Duration,
    state: State,
    toggle: button::State,
    reset: button::State,
}

enum State {
    Idle,
    Ticking { last_tick: Instant },
}

#[derive(Debug, Clone)]
enum Message {
    Toggle,
    Reset,
    Tick(Instant),
}

impl Application for Pomodoro {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Pomodoro, Command<Message>) {
        (
            Pomodoro {
                count_down: Duration::from_secs(FORTY_MINS_IN_SECS),
                state: State::Idle,
                toggle: button::State::new(),
                reset: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Pomodoro")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Toggle => match self.state {
                State::Idle => {
                    self.state = State::Ticking {
                        last_tick: Instant::now(),
                    };
                }
                State::Ticking { .. } => {
                    self.state = State::Idle;
                }
            },
            Message::Tick(now) => match &mut self.state {
                State::Ticking { last_tick } => {
                    self.count_down -= now - *last_tick;
                    *last_tick = now;
                     
                    if self.count_down.as_secs().eq(&0) {
                        self.count_down = Duration::from_secs(FORTY_MINS_IN_SECS);
                        self.state = State::Idle;
                    }
                }
                _ => {}
            },
            Message::Reset => {
                self.count_down = Duration::from_secs(FORTY_MINS_IN_SECS);
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.state {
            State::Idle => Subscription::none(),
            State::Ticking { .. } => {
                time::every(Duration::from_millis(10)).map(Message::Tick)
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        const MINUTE: u64 = 60;
        const HOUR: u64 = 60 * MINUTE;

        let seconds = self.count_down.as_secs();

        let duration = Text::new(format!(
            "{:0>2}:{:0>2}",
            (seconds % HOUR) / MINUTE,
            seconds % MINUTE,
        ))
        .size(75);

        let button = |state, label, style| {
            Button::new(
                state,
                Text::new(label)
                    .horizontal_alignment(alignment::Horizontal::Center),
            )
            .min_width(80)
            .padding(10)
            .style(style)
        };

        let toggle_button = {
            let (label, color) = match self.state {
                State::Idle => ("Start", style::Button::Primary),
                State::Ticking { .. } => ("Stop", style::Button::Secondary),
            };

            button(&mut self.toggle, label, color).on_press(Message::Toggle)
        };

        let reset_button =
            button(&mut self.reset, "Reset", style::Button::Destructive)
                .on_press(Message::Reset);

        let controls = Row::new()
            .spacing(20)
            .push(toggle_button)
            .push(reset_button);

        let content = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(duration)
            .push(controls);
    

        Container::new(content)
            .width(Length::FillPortion(50))
            .height(Length::FillPortion(150))
            .center_x()
            .center_y()
            .style(style::Container)
            .into()
    }
}

mod style {
    use iced::{button, container, Background, Color, Vector};

    pub const WINDOW_WIDTH: u32 = 400;
    pub const WINDOW_HEIGHT: u32 = 800;

    pub enum Button {
        Primary,
        Secondary,
        Destructive,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.44, 0.34, 0.89),
                    Button::Secondary => Color::from_rgb(0.59, 0.23, 0.99),
                    Button::Destructive => Color::from_rgb(0.40, 0.16, 0.66),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }

    pub struct Container;

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(Color::from_rgb(
                    0.10, 0.04, 0.16,
                ))),
                text_color: Some(Color::WHITE),
                ..container::Style::default()
            }
        }
    }
} 
