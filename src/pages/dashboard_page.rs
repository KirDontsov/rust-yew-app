use crate::{
	api::api_user_info,
	components::{Header,Section},
	router,
	store::{set_auth_user, set_page_loading, set_show_alert, Store},
};
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;
use crate::router::Route;
use yew_router::prelude::Link;

#[function_component(DashboardPage)]
pub fn dashboard_page() -> Html {
	let (store, dispatch) = use_store::<Store>();
	let user = store.auth_user.clone();
	let navigator = use_navigator().unwrap();

	use_effect_with_deps(
		move |_| {
			let dispatch = dispatch.clone();
			wasm_bindgen_futures::spawn_local(async move {
				set_page_loading(true, dispatch.clone());
				let response = api_user_info().await;
				match response {
					Ok(user) => {
						set_page_loading(false, dispatch.clone());
						set_auth_user(Some(user), dispatch);
					}
					Err(e) => {
						set_page_loading(false, dispatch.clone());
						set_show_alert(e.to_string(), dispatch);
						navigator.push(&router::Route::LoginPage);
					}
				}
			});
		},
		(),
	);

	html! {
	  <>
		<Header />
		<Section>
			<div class="mx-auto max-w-2xl py-24 sm:py-32 lg:py-48">
			  <div class="hidden sm:mb-8 sm:flex sm:justify-center">
				<div class="relative rounded-full px-3 py-1 text-sm leading-6 text-gray-300 ring-1 ring-gray-900/10 hover:ring-gray-900/20">
				  {"Объявляем о выходе на IPO. "}<a href="#" class="font-semibold text-indigo-400"><span class="absolute inset-0" aria-hidden="true"></span>{"Подробнее "}<span aria-hidden="true">{"→"}</span></a>
				</div>
			  </div>
			  <div class="text-center">
				<h1 class="text-4xl font-bold tracking-tight text-white sm:text-6xl">{"Dashboard"}</h1>
				<p class="mt-6 text-lg leading-8 text-gray-300">{"Для тех, кто хочет воспользоваться преимуществами новейших возможностей искуственного интеллекта."}</p>
				<div class="mt-10 flex items-center justify-center gap-x-6">
					<Link<Route> to={if user.is_some() { Route::ProfilePage } else {Route::LoginPage}} classes="rounded-md bg-indigo-600 hover:bg-indigo-500 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">{"Начать"}</Link<Route>>
					<a href="#" class="text-sm font-semibold leading-6 text-gray-100">{"Узнать больше "}<span aria-hidden="true">{"→"}</span></a>
				</div>
			  </div>
			</div>
		</Section>
	  </>
	}
}
