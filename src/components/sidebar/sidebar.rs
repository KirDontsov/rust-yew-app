use super::sidebar_item::SidebarItem;
use crate::{
	api::{api_logout_user, api_user_info},
	router::{self, Route},
	store::{set_auth_user, set_page_loading, set_show_alert, Store},
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(Sidebar)]
pub fn sidebar_component() -> Html {
	let (store, dispatch) = use_store::<Store>();

	let navigator = use_navigator().unwrap();
	let location = use_location().unwrap();

	let cl_dispatch = dispatch.clone();
	let cl_navigator = navigator.clone();

	let user = store.auth_user.clone();
	let user_role = match user {
		Some(u) => u.role,
		None => "".to_string(),
	};

	use_effect_with_deps(
		move |_| {
			wasm_bindgen_futures::spawn_local(async move {
				set_page_loading(true, cl_dispatch.clone());
				let response = api_user_info().await;

				match response {
					Ok(user) => {
						set_page_loading(false, cl_dispatch.clone());
						set_auth_user(Some(user), cl_dispatch);
					}
					Err(e) => {
						set_page_loading(false, cl_dispatch.clone());
						set_show_alert(e.to_string(), cl_dispatch);
						cl_navigator.push(&router::Route::LoginPage);
					}
				}
			});
		},
		(),
	);

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
						set_show_alert("Вы успешно вышли из системы".to_string(), dispatch);
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
		<aside id="default-sidebar" class="fixed top-0 left-0 z-40 w-64 h-screen transition-transform -translate-x-full sm:translate-x-0" aria-label="Sidebar">
	   <div class="h-full px-3 py-4 overflow-y-auto bg-gray-50 dark:bg-gray-800">
		  <ul class="space-y-2 font-medium">
			 <SidebarItem to={Route::DashboardPage} active={location.path() == "/dashboard"}>
			   <svg class={format!("w-5 h-5 transition duration-75 text-gray-400 {}", if location.path() == "/dashboard" {"text-indigo-400 group-hover:text-indigo-300"} else {"text-gray-400 group-hover:text-white"})} aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 20 20">
						<path d="m19.707 9.293-2-2-7-7a1 1 0 0 0-1.414 0l-7 7-2 2a1 1 0 0 0 1.414 1.414L2 10.414V18a2 2 0 0 0 2 2h3a1 1 0 0 0 1-1v-4a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v4a1 1 0 0 0 1 1h3a2 2 0 0 0 2-2v-7.586l.293.293a1 1 0 0 0 1.414-1.414Z"/>
					</svg>
			   <span class="ml-3">{"Главная"}</span>
			  </SidebarItem>
			 <SidebarItem to={Route::AccountPage} active={location.path() == "/account"}>
			   <svg class={format!("w-5 h-5 transition duration-75 text-gray-400 {}", if location.path() == "/account" {"text-indigo-400 group-hover:text-indigo-300"} else {"text-gray-400 group-hover:text-white"})} aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 14 18">
						<path d="M7 9a4.5 4.5 0 1 0 0-9 4.5 4.5 0 0 0 0 9Zm2 1H5a5.006 5.006 0 0 0-5 5v2a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-2a5.006 5.006 0 0 0-5-5Z"/>
					</svg>
			   <span class="flex-1 ml-3 whitespace-nowrap">{"Аккаунт"}</span>
			</SidebarItem>
			{ if user_role == "admin" {
				html! {
					<SidebarItem to={Route::UsersPage} active={location.path() == "/users"}>
						<svg class={format!("w-5 h-5 transition duration-75 text-gray-400 {}", if location.path() == "/users" {"text-indigo-400 group-hover:text-indigo-300"} else {"text-gray-400 group-hover:text-white"})} aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 20 19">
							<path d="M14.5 0A3.987 3.987 0 0 0 11 2.1a4.977 4.977 0 0 1 3.9 5.858A3.989 3.989 0 0 0 14.5 0ZM9 13h2a4 4 0 0 1 4 4v2H5v-2a4 4 0 0 1 4-4Z"/>
							<path d="M5 19h10v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2ZM5 7a5.008 5.008 0 0 1 4-4.9 3.988 3.988 0 1 0-3.9 5.859A4.974 4.974 0 0 1 5 7Zm5 3a3 3 0 1 0 0-6 3 3 0 0 0 0 6Zm5-1h-.424a5.016 5.016 0 0 1-1.942 2.232A6.007 6.007 0 0 1 17 17h2a1 1 0 0 0 1-1v-2a5.006 5.006 0 0 0-5-5ZM5.424 9H5a5.006 5.006 0 0 0-5 5v2a1 1 0 0 0 1 1h2a6.007 6.007 0 0 1 4.366-5.768A5.016 5.016 0 0 1 5.424 9Z"/>
						</svg>
						<span class="flex-1 ml-3 whitespace-nowrap">{"Пользователи"}</span>
					</SidebarItem>}
				} else {
					html! {<></>}
				}
			}
			<SidebarItem to={Route::QuotesPage} active={location.path() == "/quotes"}>
				<svg class={format!("w-5 h-5 transition duration-75 text-gray-400 {}", if location.path() == "/quotes" {"text-indigo-400 group-hover:text-indigo-300"} else {"text-gray-400 group-hover:text-white"})} aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 18 14">
					<path d="M6 0H2a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h4v1a3 3 0 0 1-3 3H2a1 1 0 0 0 0 2h1a5.006 5.006 0 0 0 5-5V2a2 2 0 0 0-2-2Zm10 0h-4a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h4v1a3 3 0 0 1-3 3h-1a1 1 0 0 0 0 2h1a5.006 5.006 0 0 0 5-5V2a2 2 0 0 0-2-2Z"/>
				</svg>
				<span class="flex-1 ml-3 whitespace-nowrap">{"Цитаты"}</span>
			</SidebarItem>
			<SidebarItem to={Route::MapsPage} active={location.path() == "/maps"}>
			   <svg class={format!("w-5 h-5 transition duration-75 text-gray-400 {}", if location.path() == "/maps" {"text-indigo-400 group-hover:text-indigo-300"} else {"text-gray-400 group-hover:text-white"})} aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 16 20">
						<path d="M8 0a7.992 7.992 0 0 0-6.583 12.535 1 1 0 0 0 .12.183l.12.146c.112.145.227.285.326.4l5.245 6.374a1 1 0 0 0 1.545-.003l5.092-6.205c.206-.222.4-.455.578-.7l.127-.155a.934.934 0 0 0 .122-.192A8.001 8.001 0 0 0 8 0Zm0 11a3 3 0 1 1 0-6 3 3 0 0 1 0 6Z"/>
					</svg>
			   <span class="flex-1 ml-3 whitespace-nowrap">{"Карта"}</span>
			</SidebarItem>
			<SidebarItem to={Route::FirmsPage} active={location.path() == "/crawler/firms"}>
			   <svg class={format!("w-5 h-5 transition duration-75 text-gray-400 {}", if location.path() == "/crawler/firms" {"text-indigo-400 group-hover:text-indigo-300"} else {"text-gray-400 group-hover:text-white"})} aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 18 20">
				  <path d="M17 5.923A1 1 0 0 0 16 5h-3V4a4 4 0 1 0-8 0v1H2a1 1 0 0 0-1 .923L.086 17.846A2 2 0 0 0 2.08 20h13.84a2 2 0 0 0 1.994-2.153L17 5.923ZM7 9a1 1 0 0 1-2 0V7h2v2Zm0-5a2 2 0 1 1 4 0v1H7V4Zm6 5a1 1 0 1 1-2 0V7h2v2Z"/>
			   </svg>
			   <span class="flex-1 ml-3 whitespace-nowrap">{"Парсер фирм"}</span>
			</SidebarItem>
			<SidebarItem to={Route::FirmsInfoPage} active={location.path() == "/crawler/firms_info"}>
				<svg class={format!("w-5 h-5 transition duration-75 text-gray-400 {}", if location.path() == "/crawler/firms_info" {"text-indigo-400 group-hover:text-indigo-300"} else {"text-gray-400 group-hover:text-white"})} aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 18 20">
				  <path d="M17 5.923A1 1 0 0 0 16 5h-3V4a4 4 0 1 0-8 0v1H2a1 1 0 0 0-1 .923L.086 17.846A2 2 0 0 0 2.08 20h13.84a2 2 0 0 0 1.994-2.153L17 5.923ZM7 9a1 1 0 0 1-2 0V7h2v2Zm0-5a2 2 0 1 1 4 0v1H7V4Zm6 5a1 1 0 1 1-2 0V7h2v2Z"/>
			   </svg>
				<span class="flex-1 ml-3 whitespace-nowrap">{"Парсер фирмы инфо"}</span>
			</SidebarItem>
			<SidebarItem to={Route::FirmsReviewsPage} active={location.path() == "/crawler/firms_reviews"}>
				<svg class={format!("w-5 h-5 transition duration-75 text-gray-400 {}", if location.path() == "/crawler/firms_reviews" {"text-indigo-400 group-hover:text-indigo-300"} else {"text-gray-400 group-hover:text-white"})} aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 18 20">
				  <path d="M17 5.923A1 1 0 0 0 16 5h-3V4a4 4 0 1 0-8 0v1H2a1 1 0 0 0-1 .923L.086 17.846A2 2 0 0 0 2.08 20h13.84a2 2 0 0 0 1.994-2.153L17 5.923ZM7 9a1 1 0 0 1-2 0V7h2v2Zm0-5a2 2 0 1 1 4 0v1H7V4Zm6 5a1 1 0 1 1-2 0V7h2v2Z"/>
			   </svg>
				<span class="flex-1 ml-3 whitespace-nowrap">{"Парсер фирмы отзывы"}</span>
			</SidebarItem>
				<li>
					<button onclick={handle_logout} class="flex items-center p-2 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700 group w-full">
						<svg class="flex-shrink-0 w-5 h-5 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 18 16">
						<path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1 8h11m0 0L8 4m4 4-4 4m4-11h3a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-3"/>
						</svg>
						<span class="flex-1 ml-3 whitespace-nowrap text-left">{"Выйти"}</span>
					</button>
				</li>
		  </ul>
	   </div>
	</aside>
	}
}
