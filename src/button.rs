use leptos::{html::Button, *};

const BASE_STYLE: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";

pub enum Variant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

impl Variant {
    fn as_str(&self) -> &'static str {
        match self {
            Variant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            Variant::Destructive => {
                "bg-destructive text-destructive-foreground hover:bg-destructive/90"
            }
            Variant::Outline => {
                "border border-input bg-background hover:bg-accent hover:text-accent-foreground"
            }
            Variant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
            Variant::Ghost => "hover:bg-accent hover:text-accent-foreground",
            Variant::Link => "text-primary underline-offset-4 hover:underline",
        }
    }
}

pub enum Size {
    Default,
    Small,
    Large,
    Icon,
}

impl Size {
    fn as_str(&self) -> &'static str {
        match self {
            Size::Default => "h-10 px-4 py-2",
            Size::Small => "h-9 rounded-md px-3",
            Size::Large => "h-11 rounded-md px-8",
            Size::Icon => "h-10 w-10",
        }
    }
}

pub struct ButtonConfig {
    variant: Option<Variant>,
    size: Option<Size>,
}

impl ButtonConfig {
    fn get_style(&self) -> String {
        let variant_style = match &self.variant {
            Some(variant) => variant.as_str(),
            None => Variant::Default.as_str(),
        };

        let size_style = match &self.size {
            Some(size) => size.as_str(),
            None => Size::Default.as_str(),
        };

        format!("{} {} {}", BASE_STYLE, variant_style, size_style)
    }
}

#[component]
pub fn Button(config: Option<ButtonConfig>, children: Option<Children>) -> impl IntoView {
    let config_to_use: ButtonConfig = match config {
        Some(config_param) => config_param,
        None => ButtonConfig {
            variant: Some(Variant::Default),
            size: Some(Size::Default),
        },
    };
    let style_string: String = config_to_use.get_style();
    match children {
        Some(optionalChildren) => {
            view! {
                <div class=style_string>
                {optionalChildren()}
                </div>
            }
        }
        None => view! {
            <div>
                <button class=style_string>
                </button>
            </div>
        },
    }
}
