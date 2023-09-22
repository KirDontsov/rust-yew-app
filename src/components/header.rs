use crate::{
	api::user_api::api_logout_user,
	router::{self, Route},
	store::{set_auth_user, set_page_loading, set_show_alert, Store},
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(Header)]
pub fn header_component() -> Html {
	let (store, dispatch) = use_store::<Store>();
	let user = store.auth_user.clone();
	let navigator = use_navigator().unwrap();

	let handle_logout = {
		let store_dispatch = dispatch.clone();
		let cloned_navigator = navigator.clone();

		Callback::from(move |_: MouseEvent| {
			let dispatch = store_dispatch.clone();
			let navigator = cloned_navigator.clone();
			spawn_local(async move {
				set_page_loading(true, dispatch.clone());
				let res = api_logout_user().await;
				match res {
					Ok(_) => {
						set_page_loading(false, dispatch.clone());
						set_auth_user(None, dispatch.clone());
						set_show_alert("Logged out successfully".to_string(), dispatch);
						navigator.push(&router::Route::LoginPage);
					}
					Err(e) => {
						set_show_alert(e.to_string(), dispatch.clone());
						set_page_loading(false, dispatch);
					}
				};
			});
		})
	};

	html! {
		<header class="h-10 z-10 absolute w-full bg-transparent">
		<nav class="h-full flex justify-between container items-center">
		  <div>
			<Link<Route> to={Route::HomePage} classes="text-white">{"Paradox"}</Link<Route>>
		  </div>
		  <ul class="flex items-center gap-4">
			<li>
			  <Link<Route> to={Route::HomePage} classes="text-white">{"Home"}</Link<Route>>
			</li>
			if user.is_some() {
			   <>
				<li>
				  <Link<Route> to={Route::ProfilePage} classes="text-white">{"Profile"}</Link<Route>>
				</li>
				<li
				  class="cursor-pointer text-white"
				>
				  {"Create Post"}
				</li>
				<li class="cursor-pointer text-white" onclick={handle_logout}>
				  {"Logout"}
				</li>
			  </>

			} else {
			  <>
				<li>
				  <Link<Route> to={Route::RegisterPage} classes="text-white">{"SignUp"}</Link<Route>>
				</li>
				<li>
				  <Link<Route> to={Route::LoginPage} classes="text-white">{"Login"}</Link<Route>>
				</li>
			  </>
			}
		  </ul>
		</nav>
	  </header>
	}
}
