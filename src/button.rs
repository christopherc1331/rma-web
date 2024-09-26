use leptos::*;

const BASE_STYLE: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";

pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

impl ButtonVariant {
    fn as_str(&self) -> &'static str {
        match self {
            ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            ButtonVariant::Destructive => {
                "bg-destructive text-destructive-foreground hover:bg-destructive/90"
            }
            ButtonVariant::Outline => {
                "border border-input bg-background hover:bg-accent hover:text-accent-foreground"
            }
            ButtonVariant::Secondary => {
                "bg-secondary text-secondary-foreground hover:bg-secondary/80"
            }
            ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
            ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
        }
    }
}

pub enum ButtonSize {
    Default,
    Small,
    Large,
    Icon,
}

impl ButtonSize {
    fn as_str(&self) -> &'static str {
        match self {
            ButtonSize::Default => "h-10 px-4 py-2",
            ButtonSize::Small => "h-9 rounded-md px-3",
            ButtonSize::Large => "h-11 rounded-md px-8",
            ButtonSize::Icon => "h-10 w-10",
        }
    }
}

#[component]
pub fn Button(
    #[prop(default = ButtonVariant::Default)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Default)] size: ButtonSize,
    #[prop(default = "")] label: &'static str,
    #[prop(default = "")] class: &'static str,
    children: Children,
) -> impl IntoView {
    let style_string: String = format!("{} {} {}", BASE_STYLE, variant.as_str(), size.as_str());
    let combined_styles = format!("{} {}", style_string, class);
    view! {
        <div class=combined_styles>
            {label}
            {children()}
        </div>
    }
}
