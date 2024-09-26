use leptos::*;

#[derive(Clone)]
pub enum InputType {
    Button,
    Checkbox,
    Color,
    Date,
    DateTimeLocal,
    Email,
    File,
    Hidden,
    Image,
    Month,
    Number,
    Password,
    Radio,
    Range,
    Reset,
    Search,
    Submit,
    Tel,
    Text,
    Time,
    Url,
    Week,
}

impl InputType {
    fn as_str(&self) -> String {
        match self {
            InputType::Button => "button",
            InputType::Checkbox => "checkbox",
            InputType::Color => "color",
            InputType::Date => "date",
            InputType::DateTimeLocal => "datetime-local",
            InputType::Email => "email",
            InputType::File => "file",
            InputType::Hidden => "hidden",
            InputType::Image => "image",
            InputType::Month => "month",
            InputType::Number => "number",
            InputType::Password => "password",
            InputType::Radio => "radio",
            InputType::Range => "range",
            InputType::Reset => "reset",
            InputType::Search => "search",
            InputType::Submit => "submit",
            InputType::Tel => "tel",
            InputType::Text => "text",
            InputType::Time => "time",
            InputType::Url => "url",
            InputType::Week => "week",
        }
        .to_string()
    }
}

#[component]
pub fn Input(
    #[prop(default = InputType::Text)] input_type: InputType,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] placeholder: &'static str,
) -> impl IntoView {
    let input_class = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50".to_string();
    let class_to_use = format!("{} {}", input_class, class);
    view! {
            <input type=input_type.as_str() placeholder=placeholder  class=class_to_use/>
    }
}
