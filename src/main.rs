use iced::{
    button, text_input, Button, Column, Row, Element, Length, Sandbox,
    Settings, Text, Container
};

pub fn main() {
    AssStore::run(Settings::default());
}

#[derive(Default)]
struct AssStore {
    search_input: text_input::State,
    search_submit_btn: iced::button::State,
    search_text: String,
}

//#[derive(Debug, Clone, Copy)]
#[derive(Debug, Clone)]
enum Message {
    SearchTextChanged(String),
    SearchButtonClicked,
}

impl Sandbox for AssStore {
    type Message = Message;

    fn new() -> Self {
        AssStore::default()
    }

    fn title(&self) -> String {
        String::from("Ass Store")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SearchTextChanged(text) => self.search_text = text,
            Message::SearchButtonClicked => println!("pesquisar"),
        }
    }

    fn view(&mut self) -> Element<Message> {

        let text_input = text_input::TextInput::new(&mut self.search_input, "Digite aqui", &self.search_text, Message::SearchTextChanged)
            .padding(10)
            ;
        
        let submit_btn = Button::new(&mut self.search_submit_btn, Text::new("Vai!"))
            .on_press(Message::SearchButtonClicked)
            .padding(10)
            ;
        
        let search_row = Row::new()
            .spacing(5)
            .push(text_input)
            .push(submit_btn)
            ;
        
        let content = Column::new()
            .spacing(20)
            .push(Text::new("Ass Store").size(50))
            .push(Text::new("uma loja de apps linux funcional."))
            .push(search_row)
            ;

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            //.center_y()
            .into()
    }
}
