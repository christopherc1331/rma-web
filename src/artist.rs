use leptos::{
    component, create_resource, server, view, CollectView, IntoView, Params, ServerFnError,
    SignalGet, SignalWith, Transition,
};
use leptos_router::{use_params, Params};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SocialLink {
    icon_url: String,
    url: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct PortfolioImage {
    url: String,
    bio: Option<String>,
    date: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ArtistInfo {
    id: String,
    img_url: String,
    name: String,
    bio: String,
    links: Option<Vec<SocialLink>>,
    portfolio: Option<Vec<PortfolioImage>>,
}

#[server(FetchArtist)]
async fn fetch_artist(id: String) -> Result<ArtistInfo, ServerFnError> {
    // TODO: temp; will call mongo eventually
    Ok(ArtistInfo {
        id,
        img_url: "https://img.daisyui.com/images/stock/photo-1635805737707-575885ab0820.webp".into(),
        name: "Lyndon Korcala".into(),
        bio: "Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem quasi. In deleniti eaque aut repudiandae et a id nisi.".into(),
        portfolio: Some(vec![
            PortfolioImage {
                url: "https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp".into(),
                bio: Some("Provident cupiditate voluptatem et in.".into()),
                date: "2/21/14".into(),
                ..Default::default()
            },
            PortfolioImage {
                url: "https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp".into(),
                date: "2/22/14".into(),
                ..Default::default()
            },
            PortfolioImage {
                url: "https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp".into(),
                bio: Some("Provident cupiditate voluptatem et in.".into()),
                date: "2/25/14".into(),
                ..Default::default()
            },
            PortfolioImage {
                url: "https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp".into(),
                bio: Some("Provident cupiditate voluptatem et in.".into()),
                date: "2/27/14".into(),
                ..Default::default()
            },
            PortfolioImage {
                url: "https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp".into(),
                date: "2/28/14".into(),
                ..Default::default()
            },
            PortfolioImage {
                url: "https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp".into(),
                date: "3/4/14".into(),
                ..Default::default()
            },
        ]),
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
                <div class="flex flex-wrap-auto">
                    {
                        match artist_info.portfolio {
                            None => view!{""}.into_view(),
                            Some(portfolio) => portfolio.into_iter()
                                .map(|portfolio_item| view!{
                                    <div class="card bg-base-100 w-96 shadow-xl">
                                        <figure>
                                            <img src=portfolio_item.url alt="Shoes" />
                                        </figure>
                                        <div class="card-body">
                                            <h2 class="card-title">{portfolio_item.date}</h2>
                                            <p>{portfolio_item.bio}</p>
                                            <div class="card-actions justify-end">
                                                <button class="btn btn-primary">Buy Now</button>
                                            </div>
                                        </div>
                                    </div>
                                }.into_view()
                            ).collect_view()
                        }
                    }
                </div>
             }.into_view()
        }}
        </Transition>
    }
}
