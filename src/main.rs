use iced::{
    button, executor, pick_list, text_input, window, Alignment, Application, Button, Column,
    Command, Element, Length, PickList, Row, Settings, Text, TextInput,
};
use regex::Regex;
use serde::Deserialize;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Currencies {
    USD,
    EUR,
    RUB,
    GBP,
    CAD,
    AUD,
    JPY,
    INR,
    NZD,
    CHF,
}

const CURRENCIES: [Currencies; 10] = [
    Currencies::USD,
    Currencies::EUR,
    Currencies::RUB,
    Currencies::GBP,
    Currencies::CAD,
    Currencies::AUD,
    Currencies::JPY,
    Currencies::INR,
    Currencies::NZD,
    Currencies::CHF,
];

struct CurrencyConverter {
    convert_button: button::State,
    amount_input: text_input::State,
    from_select: pick_list::State<Currencies>,
    to_select: pick_list::State<Currencies>,
    result: String,
    amount: String,
    from: Option<Currencies>,
    to: Option<Currencies>,
}

#[derive(Debug, Clone, Deserialize)]
struct Query {
    from: String,
    to: String,
    amount: i64,
}
#[derive(Debug, Clone, Deserialize)]
struct Currency {
    success: bool,
    query: Query,
    result: f64,
}

#[derive(Debug, Clone)]
enum Message {
    Convert,
    Amount(String),
    From(Currencies),
    To(Currencies),
    Response(Currency),
}

const API: &str = "https://api.apilayer.com/exchangerates_data/convert";

async fn convert(from: String, to: String, amount: String) -> Currency {
    let url = format!(
        "{}?to={}&from={}&amount={}&apikey={}",
        API,
        currency_code(&to),
        currency_code(&from),
        amount,
        "6Hi3VL1ZsP2DnGMZmfZZJSiA0Wvzn49X"
    );

    reqwest::get(&url)
        .await
        .unwrap()
        .json::<Currency>()
        .await
        .unwrap()
}

fn currency_code(str: &str) -> &str {
    let regex = Regex::new(r"^[A-Z]{3}").unwrap();
    regex
        .captures(str)
        .unwrap()
        .get(0)
        .unwrap()
        .to_owned()
        .as_str()
}

impl Application for CurrencyConverter {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                convert_button: Default::default(),
                amount_input: text_input::State::new(),
                from_select: pick_list::State::new(),
                to_select: pick_list::State::new(),
                result: String::from(""),
                amount: String::from("1.00"),
                from: Some(Currencies::RUB),
                to: Some(Currencies::USD),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Currency converter")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Convert => {
                self.result = String::from("Loading...");
                let from = self.from.unwrap().to_string().to_owned();
                let to = self.to.unwrap().to_string().to_owned();
                let amount = self.amount.clone();

                Command::perform(convert(from, to, amount), Message::Response)
            }
            Message::Amount(amount) => {
                let regex = Regex::new(r"^([0-9]+)?(\.)?([0-9]+)?$").unwrap();
                if regex.is_match(amount.as_str()) {
                    self.amount = amount;
                }
                Command::none()
            }
            Message::From(currency) => {
                if currency == self.to.unwrap() {
                    self.to = self.from;
                }
                self.from = Some(currency);
                Command::none()
            }
            Message::To(currency) => {
                if currency == self.from.unwrap() {
                    self.from = self.to;
                }
                self.to = Some(currency);
                Command::none()
            }
            Message::Response(currency) => {
                if currency.success {
                    self.result = format!(
                        "{} {} = {} {}",
                        currency.query.amount,
                        currency.query.from,
                        currency.result,
                        currency.query.to
                    )
                }
                Command::none()
            }
        }
    }
    fn view(&mut self) -> Element<Message> {
        let amount_label = Text::new("Amount").size(20);
        let amount_input =
            TextInput::new(&mut self.amount_input, "", &self.amount, Message::Amount)
                .width(Length::from(320))
                .padding(12)
                .size(16);
        let amount_column = Column::new().push(amount_label).push(amount_input);

        let from_label = Text::new("From").size(20);
        let from_select = PickList::new(
            &mut self.from_select,
            &CURRENCIES[..],
            self.from,
            Message::From,
        )
        .width(Length::from(320))
        .padding(12)
        .text_size(16);
        let from_column = Column::new()
            .push(from_label)
            .push(from_select)
            .padding([0, 16]);

        let to_label = Text::new("To").size(20);
        let to_select = PickList::new(&mut self.to_select, &CURRENCIES[..], self.to, Message::To)
            .width(Length::from(320))
            .padding(12)
            .text_size(16);
        let to_column = Column::new().push(to_label).push(to_select);

        let inputs_row = Row::new()
            .push(amount_column)
            .push(from_column)
            .push(to_column);

        let result = Text::new(&self.result).size(50);
        let result_row = Row::new().push(result).padding(20);

        let convert_button = Button::new(&mut self.convert_button, Text::new("Convert"))
            .on_press(Message::Convert)
            .padding([12, 35]);

        let info_row = Column::new()
            .push(convert_button)
            .push(result_row)
            .align_items(Alignment::Center)
            .padding([20, 0]);

        Column::new()
            .padding(50)
            .push(inputs_row)
            .push(info_row)
            .align_items(Alignment::Center)
            .into()
    }
}

fn main() -> iced::Result {
    CurrencyConverter::run(Settings {
        antialiasing: true,
        window: window::Settings {
            position: window::Position::Centered,
            size: (1150, 350),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

impl Display for Currencies {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(
            formatter,
            "{}",
            match self {
                Currencies::RUB => "RUB - Russian Ruble",
                Currencies::EUR => "EUR - Euro",
                Currencies::USD => "USD - US Dollar",
                Currencies::GBP => "GBP - British Pound",
                Currencies::CAD => "CAD - Canadian Dollar",
                Currencies::AUD => "AUD - Australian Dollar",
                Currencies::JPY => "JPY - Japanese Yen",
                Currencies::NZD => "NZD - New Zealand Dollar",
                Currencies::INR => "INR - Indian Rupee",
                Currencies::CHF => "CHF - Swiss Franc",
            }
        )
    }
}
