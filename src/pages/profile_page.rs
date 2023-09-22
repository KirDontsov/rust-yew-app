use crate::{
	api::user_api::api_user_info,
	components::header::Header,
	components::section::Section,
	router,
	store::{set_auth_user, set_page_loading, set_show_alert, Store},
};
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
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
			<div class="w-full px-32">
			  <div class="px-4 sm:px-0">
			    <h3 class="text-base font-semibold leading-7 text-gray-100">{"Profile Information"}</h3>
			    <p class="mt-1 max-w-2xl text-sm leading-6 text-gray-300">{"Personal details."}</p>
			  </div>
			  if let Some(user) = user {
					<div class="mt-6 border-t border-gray-100">
			    <dl class="divide-y divide-gray-100">
			      <div class="px-4 py-6 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-0">
			        <dt class="text-sm font-medium leading-6 text-gray-100">{"Full name"}</dt>
			        <dd class="mt-1 text-sm leading-6 text-gray-300 sm:col-span-2 sm:mt-0">{format!("Name: {}", user.name)}</dd>
			      </div>
			      <div class="px-4 py-6 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-0">
			        <dt class="text-sm font-medium leading-6 text-gray-100">{"User ID"}</dt>
			        <dd class="mt-1 text-sm leading-6 text-gray-300 sm:col-span-2 sm:mt-0">{user.id}</dd>
			      </div>
			      <div class="px-4 py-6 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-0">
			        <dt class="text-sm font-medium leading-6 text-gray-100">{"Email address"}</dt>
			        <dd class="mt-1 text-sm leading-6 text-gray-300 sm:col-span-2 sm:mt-0">{user.email}</dd>
			      </div>
			      <div class="px-4 py-6 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-0">
			        <dt class="text-sm font-medium leading-6 text-gray-100">{"Role"}</dt>
			        <dd class="mt-1 text-sm leading-6 text-gray-300 sm:col-span-2 sm:mt-0">{user.role}</dd>
			      </div>
			    </dl>
			  </div>
				} else {
				  <p class="mb-4">{"Loading..."}</p>
				}
			</div>
	  </Section>
	</>
	}
}
