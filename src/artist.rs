use leptos::{
    component, create_resource, server, view, IntoView, Params, ServerFnError, SignalGet, SignalWith, Transition
};
use leptos_router::{use_params, Params};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SocialLink {
    icon_url: String,
    url: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ArtistInfo {
    id: String,
    img_url: String,
    name: String,
    bio: String,
    links: Option<Vec<SocialLink>>,
}

#[server(FetchArtist)]
async fn fetch_artist(id: String) -> Result<ArtistInfo, ServerFnError> {
    // TODO: temp; will call mongo eventually
    Ok(ArtistInfo {
        id,
        img_url: "https://img.daisyui.com/images/stock/photo-1635805737707-575885ab0820.webp".into(),
        name: "Lyndon Korcala".into(),
        bio: "Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem quasi. In deleniti eaque aut repudiandae et a id nisi.".into(),
        ..Default::default()
    })
}

#[derive(Params, PartialEq, Default, Clone)]
struct ArtistParams {
    id: Option<String>,
}

#[component]
pub fn ArtistPage() -> impl IntoView {
    let params = use_params::<ArtistParams>();
     let id = params.with(|params| {
         params
             .as_ref()
             .map(|params| params.clone().id)
             .unwrap_or_default()
             .unwrap_or_default()
     });
    let artist_resource = create_resource(
        || (),
        move |_| {
            let id = id.clone();
            async move { fetch_artist(id).await.unwrap() }
        },
    );
    view! {
        <Transition
            fallback=move || view! {<p>"Loading..."</p>}
        >
        {move || match artist_resource.get() {
             // TODO: Redirect to 404 page on none
             None => view!{<p>"Error encountered, artist not found"</p>}.into_view(),
             Some(artist_info) => view! {
                <div class="hero bg-base-200">
                    <div class="hero-content flex-col lg:flex-row-reverse">
                        <img src=artist_info.img_url class="max-w-sm rounded-lg shadow-2xl" />
                        <div>
                            <h1 class="text-5xl font-bold">{artist_info.name}</h1>
                            <p class="py-6">{artist_info.bio}</p>
                            <button class="btn btn-primary">Book Now</button>
                        </div>
                    </div>
                </div>
             }.into_view()
        }}
        </Transition>
    }
}
