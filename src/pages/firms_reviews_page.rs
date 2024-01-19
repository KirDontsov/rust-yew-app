use crate::{
	api::api_parse_firms_reviews,
	components::{Section, Sidebar},
	store::{set_page_loading, set_show_alert, Store},
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(FirmsReviewsPage)]
pub fn firms_reviews_page() -> Html {
	let (_, dispatch) = use_store::<Store>();
	let cl_dispatch = dispatch.clone();

	use_effect_with_deps(
		move |_| {
			wasm_bindgen_futures::spawn_local(async move {
				set_page_loading(true, cl_dispatch.clone());
				let response: Result<_, _> = api_parse_firms_reviews().await;
				match response {
					Ok(_) => {
						set_page_loading(false, cl_dispatch.clone());
					}
					Err(e) => {
						set_page_loading(false, cl_dispatch.clone());
						set_show_alert(e.to_string(), cl_dispatch);
					}
				}
			});
		},
		(),
	);

	html! {
	  <>
			<Sidebar />
			<Section>
				<div class="text-center">
					<h1 class="text-4xl font-bold tracking-tight text-white sm:text-6xl">{"Firms reviews"}</h1>
					<p class="mt-6 text-lg leading-8 text-gray-300">{"Для тех, кто хочет воспользоваться преимуществами новейших возможностей искуственного интеллекта."}</p>
			  </div>
			</Section>
	  </>
	}
}
