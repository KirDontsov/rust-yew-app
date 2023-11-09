use crate::components::{HeroHeader, Section};
use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::store::Store;
use yewdux::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
	let (store, _) = use_store::<Store>();
	let user = store.auth_user.clone();
	html! {
	  <>
		<HeroHeader />
		<Section p="pl-0">
			<div class="mx-auto max-w-2xl py-24 sm:py-32 lg:py-48">
			  <div class="hidden sm:mb-8 sm:flex sm:justify-center">
					<div class="relative rounded-full px-3 py-1 text-sm leading-6 text-gray-300 ring-1 ring-gray-900/10 hover:ring-gray-900/20">
					  {"Объявляем о выходе на IPO. "}<a href="#" class="font-semibold text-indigo-400"><span class="absolute inset-0" aria-hidden="true"></span>{"Подробнее "}<span aria-hidden="true">{"→"}</span></a>
					</div>
			  </div>
			  <div class="text-center">
					<h1 class="text-4xl font-bold tracking-tight text-white sm:text-6xl">{"Данные для масштабирования вашего онлайн-бизнеса"}</h1>
					<p class="mt-6 text-lg leading-8 text-gray-300">{"Для тех, кто хочет воспользоваться преимуществами новейших возможностей искуственного интеллекта."}</p>
					<div class="mt-10 flex items-center justify-center gap-x-6">
						<Link<Route> to={if user.is_some() { Route::AccountPage } else {Route::LoginPage}} classes="rounded-md bg-indigo-600 hover:bg-indigo-500 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">{"Начать"}</Link<Route>>
						<a href="#" class="text-sm font-semibold leading-6 text-gray-100">{"Узнать больше "}<span aria-hidden="true">{"→"}</span></a>
					</div>
			  </div>
			</div>
		</Section>
	  </>
	}
}
