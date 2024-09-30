use leptos::{component, view, IntoView, Params, SignalWith};
use leptos_router::{use_params, Params};

#[derive(Params, PartialEq)]
struct ArtistParams {
    id: Option<String>,
}

#[component]
pub fn ArtistPage() -> impl IntoView {
    let params = use_params::<ArtistParams>();
    let id = move || {
        params.with(|params| {
            params.as_ref()
                .map(|params| params.id)
                .unwrap_or_default()
        })
    };
    view! {
        <div class="hero bg-base-200">
            <div class="hero-content flex-col lg:flex-row-reverse">
                <img
                src="https://img.daisyui.com/images/stock/photo-1635805737707-575885ab0820.webp"
                class="max-w-sm rounded-lg shadow-2xl" />
                <div>
                <h1 class="text-5xl font-bold">Box Office News!</h1>
                <p class="py-6">
                    Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem
                    quasi. In deleniti eaque aut repudiandae et a id nisi.
                </p>
                <button class="btn btn-primary">Book Now</button>
                </div>
            </div>
        </div>
    }
}
