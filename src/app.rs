use crate::{landing::Landing, search_form::SearchForm, artist::ArtistPage, nav::NavTop};
use leptos::*;
use leptos_material::{components::icon::Icon, UseMaterialWebComponents};
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/rma-web.css" />
        <Title text="RateMyArtist" />
        <UseMaterialWebComponents />
        <Router>
            <main class="container mx-auto">
                <NavTop/>
                <Routes>
                    <Route path="" view=HomePage />
                    <Route path="landing" view=Landing />
                    <Route path="/*any" view=NotFound />
                    <Route path="artist/:id?" view=ArtistPage />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);
    let search_val = create_rw_signal("".to_string());

    view! {
        <div class="home">
            <header class="nav">
                <div class="branding">
                    <Icon name="flare"/>
                    <h1>RateMyArtist</h1>
                </div>
                <ul>
                    <li>Home</li>
                    <li>Artists</li>
                    <li>Reviews</li>
                    <li>About</li>
                </ul>
            </header>
            <div class="hero">
                <h1>Find Your Perfect Tattoo Artist</h1>
                <p>Discover, rate, and review tattoo artists based on customer service and tattoo quality. Share your experience and help others find their ideal artist.</p>
                <div class="search-container">
                    <SearchForm />
                    </div>
            </div>
            <h1>"Welcome to Leptos!"</h1>
            <button on:click=on_click>"Click Me: " {count}</button>
            <h1>{"Hello from leptos-material!"}</h1>
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Not Found"</h1> }
}
