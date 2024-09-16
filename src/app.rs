use leptos::*;
use leptos_material::{
    components::{
        button::{Button, ButtonType},
        icon::Icon,
        iconbutton::{IconButton, IconButtonStyle},
        textfield::TextField,
    },
    UseMaterialWebComponents,
};
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/rma-web.css" />
        <Link href="https://fonts.googleapis.com/icon?family=Material+Icons"/>
        <Title text="RateMyArtist" />
        <UseMaterialWebComponents />
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage />
                    <Route path="/*any" view=NotFound />
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
                    <TextField attr:label="Search for artists or styles" value=search_val/>
                <IconButton
                    style=IconButtonStyle::Filled
                    button_type=ButtonType::Submit
                >
            <span class="material-icons">favorite</span></IconButton>
                <Button>Submit</Button>
                    </div>
            </div>
            <h1>"Welcome to Leptos!"</h1>
            <button on:click=on_click>"Click Me: " {count}</button>
            <h1>{"Hello from leptos-material!"}</h1>
            <div class="wrapper px-2 w-full">
            <form
                action=""
                class="max-w-sm bg-gray-100 px-3 py-5 rounded shadow-lg my-10 m-auto"
            >
                <div class="flex flex-col space-y-3">
                <div
                    class="flex items-center bg-white border border-gray-100 rounded px-2"
                >
                    <svg
                        fill="currentColor"
                        viewBox="0 0 20 20"
                        class="h-6 text-gray-500 m-0 mr-1"
                    >
                        <path
                            d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z"
                        ></path>
                        <path
                            d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"
                        ></path>
                    </svg>
                    <input
                        type="text"
                        placeholder="Email"
                        class="w-full py-2 px-1 placeholder-indigo-400 outline-none placeholder-opacity-50"
                        autocomplete="off"
                    />
                </div>
                <button
                    type="submit"
                    class="text-white bg-indigo-500 px-4 py-2 rounded"
                >
                    Enviar
                </button>
                </div>
            </form>
        </div>
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
