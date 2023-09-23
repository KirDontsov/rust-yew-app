use crate::{
	components::Header,
	components::Section,
	components::Spinner,
	store::{Store},
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
	let (store, dispatch) = use_store::<Store>();
	let user = store.auth_user.clone();

	html! {
	<>
	  <Header />
	  <Section>
			<div class="w-full px-32">
			  <div class="px-4 sm:px-0">
				<h3 class="text-base font-semibold leading-7 text-gray-100">{"Аккаунт"}</h3>
				<p class="mt-1 max-w-2xl text-sm leading-6 text-gray-300">{"Личная информация."}</p>
			  </div>
			  if let Some(user) = user {
					<div class="mt-6 border-t border-gray-100">
				<dl class="divide-y divide-gray-100">
				  <div class="px-4 py-6 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-0">
					<dt class="text-sm font-medium leading-6 text-gray-100">{"Имя"}</dt>
					<dd class="mt-1 text-sm leading-6 text-gray-300 sm:col-span-2 sm:mt-0">{user.name}</dd>
				  </div>
				  <div class="px-4 py-6 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-0">
					<dt class="text-sm font-medium leading-6 text-gray-100">{"Id пользователя"}</dt>
					<dd class="mt-1 text-sm leading-6 text-gray-300 sm:col-span-2 sm:mt-0">{user.id}</dd>
				  </div>
				  <div class="px-4 py-6 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-0">
					<dt class="text-sm font-medium leading-6 text-gray-100">{"Email адрес"}</dt>
					<dd class="mt-1 text-sm leading-6 text-gray-300 sm:col-span-2 sm:mt-0">{user.email}</dd>
				  </div>
				  <div class="px-4 py-6 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-0">
					<dt class="text-sm font-medium leading-6 text-gray-100">{"Роль"}</dt>
					<dd class="mt-1 text-sm leading-6 text-gray-300 sm:col-span-2 sm:mt-0">{user.role}</dd>
				  </div>
				</dl>
			  </div>
				} else {
				  <p class="mb-4"><Spinner /></p>
				}
			</div>
	  </Section>
	</>
	}
}
