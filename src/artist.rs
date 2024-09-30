use leptos::{component, view, server, ServerFnError, IntoView, Params, SignalWith};
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

//impl Default for ArtistInfo {
//    fn default() -> Self {
//        ArtistInfo {
//            links: None,
//        }
//    }
//}

#[server(FetchArtist)]
async fn fetch_artist(id: String) -> Result<ArtistInfo, ServerFnError> {
    // TODO: temp; will call mongo eventually
    Ok(ArtistInfo {
        id: id.into(),
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
    let id =  params.with(|params|
        params.as_ref()
            .map(|params| params.clone().id)
            .unwrap_or_default()
        .unwrap_or_default());
    let artist_info = fetch_artist(id);
    view! {
        <div class="hero bg-base-200">
            <div class="hero-content flex-col lg:flex-row-reverse">
                <img
                src="foobae"
                class="max-w-sm rounded-lg shadow-2xl" />
                <div>
                <h1 class="text-5xl font-bold">Box Office News!</h1>
                <p class="py-6">
                    
                </p>
                <button class="btn btn-primary">Book Now</button>
                </div>
            </div>
        </div>
    }
}
