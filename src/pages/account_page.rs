use crate::{
	components::{ContentSection, Header, Section, Sidebar, Spinner},
	store::Store,
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(AccountPage)]
pub fn account_page() -> Html {
	let (store, _) = use_store::<Store>();
	let user = store.auth_user.clone();

	html! {
	<>
	  <Sidebar />
	  <Section>
			<Header title="Аккаунт" sub_title="Личная информация." />
			<ContentSection h="200">
			  <div class="px-4 sm:px-0">
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
			</ContentSection>
	  </Section>
	</>
	}
}
