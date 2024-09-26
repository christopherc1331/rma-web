use leptos::*;

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

#[component]
pub fn Button(
    #[prop(default = Variant::Default)] variant: Variant,
    #[prop(default = Size::Default)] size: Size,
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
