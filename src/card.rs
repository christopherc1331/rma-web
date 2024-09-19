use leptos::*;

#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! {
        <div class="rounded-lg border bg-card text-card-foreground shadow-sm">{children()}</div>
    }
}

#[component]
pub fn CardHeader(children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col space-y-1.5 p-6">{children()}</div>
    }
}

#[component]
pub fn CardTitle(children: Children) -> impl IntoView {
    view! {
        <h3 class="text-2xl font-semibold leading-none tracking-tight">{children()}</h3>
    }
}

#[component]
pub fn CardDescription(children: Children) -> impl IntoView {
    view! {
        <p class="text-sm text-muted-foreground">{children()}</p>
    }
}

#[component]
pub fn CardContent(children: Children) -> impl IntoView {
    view! {
        <div class="p-6 pt-0">{children()}</div>
    }
}

#[component]
pub fn CardFooter(children: Children) -> impl IntoView {
    view! {
        <div class="flex items-center p-6 pt-0">{children()}</div>
    }
}
