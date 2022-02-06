pub mod storage;

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

const POMODORO_TIME: u64 = 1500;
const SHORT_BREAK_TIME: u64 = 300;
const LONG_BREAK_TIME: u64 = 900;

struct Pomodoro {
    storage: storage::Storage,
    count_down: Duration,
    state: State,
    toggle: button::State,
    reset: button::State,
    set_time: button::State,
    pomodoro: button::State,
    short_break: button::State,
    long_break: button::State,
}

enum State {
    Idle,
    Ticking { last_tick: Instant },
}

#[derive(Debug, Clone)]
enum Message {
    Toggle,
    Reset,
    SetTime,
    Pomodoro,
    ShortBreak,
    LongBreak,
    Tick(Instant),
}

impl Application for Pomodoro {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Pomodoro, Command<Message>) {
        (
            Pomodoro {
                storage: storage::Storage::new("storage.json").expect("Storage"),
                count_down: Duration::from_secs(POMODORO_TIME),
                state: State::Idle,
                toggle: button::State::new(),
                reset: button::State::new(),
                set_time: button::State::new(),
                pomodoro: button::State::new(),
                short_break: button::State::new(),
                long_break: button::State::new(),
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

                    // let activity: f32 = user.activity.parse().expect("PARSE");

                    // user.activity = format!("{}", self.count_down.as_secs_f32() + activity);

                    // self.storage.update(&user).expect("update UPDATE");

                    if self.count_down.as_secs().eq(&0) {
                        self.count_down = Duration::from_secs(POMODORO_TIME);
                        self.state = State::Idle;
                    }
                }
                _ => {}
            },
            Message::Reset => {
                self.count_down = Duration::from_secs(POMODORO_TIME);
            },
            Message::SetTime => {
            },
            Message::Pomodoro => {
                self.count_down = Duration::from_secs(POMODORO_TIME);
            },
            Message::ShortBreak => {
                self.count_down = Duration::from_secs(SHORT_BREAK_TIME);
            },
            Message::LongBreak => {
                self.count_down = Duration::from_secs(LONG_BREAK_TIME);
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

        let user = storage::User{username: "aalt".to_string(), activity: "2.5".to_string()};
        self.storage.update(&user).expect("view UPDATE");

        let seconds = self.count_down.as_secs();

        let duration = Text::new(format!(
            "{:0>2}:{:0>2}",
            (seconds % HOUR) / MINUTE,
            seconds % MINUTE,
        ))
        .size(75);

        // let user = self.storage.get().expect("view GET");
        // let activity = Text::new(user.activity)
        //     .size(15);

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

        let pomodoro_button = 
            button(&mut self.pomodoro, "Pomodoro", style::Button::Pomodoro)
                .on_press(Message::Pomodoro);

        let short_break_button = 
            button(&mut self.short_break, "Short break", style::Button::ShortBreak)
                .on_press(Message::ShortBreak);

        let long_break_button = 
            button(&mut self.long_break, "Long break", style::Button::LongBreak)
                .on_press(Message::LongBreak);

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

        let set_time_button = 
            button(&mut self.set_time, "Set time", style::Button::SetTime)
                .on_press(Message::SetTime);
        
        let up_controls = Row::new()
            .spacing(30)
            .push(pomodoro_button)
            .push(short_break_button)
            .push(long_break_button);

        let down_controls = Row::new()
            .spacing(20)
            .push(toggle_button)
            .push(reset_button)
            .push(set_time_button);

        let content = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(up_controls)
            .push(duration)
            .push(down_controls);
    

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
        Pomodoro,
        ShortBreak,
        LongBreak,
        Primary,
        Secondary,
        Destructive,
        SetTime,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.44, 0.34, 0.89),
                    Button::Secondary => Color::from_rgb(0.59, 0.23, 0.99),
                    Button::Destructive => Color::from_rgb(0.40, 0.16, 0.66),
                    Button::SetTime => Color::from_rgb(0.15, 0.87, 0.78),
                    Button::Pomodoro | Button::ShortBreak | Button::LongBreak => Color::from_rgb(0.0, 0.0, 0.0), 
                    
                })),
                border_radius: match self {
                    Button::Primary | Button::Secondary | Button::Destructive => 11.0,
                    Button::SetTime | Button::Pomodoro | Button::ShortBreak | Button::LongBreak => 5.0,
                },
                shadow_offset: match self {
                    Button::Primary | Button::Secondary | Button::Destructive => Vector::new(1.0, 1.0),
                    Button::SetTime | Button::Pomodoro | Button::ShortBreak | Button::LongBreak => Vector::new(0.5, 0.5),
                },
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
