use crate::{
	api::api_get_users,
	components::{ContentSection, Header, Pagination, Section, Sidebar, Spinner},
	store::{set_page_loading, set_show_alert, Store},
};
use std::primitive::*;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(UsersPage)]
pub fn users_page() -> Html {
	let users = use_state(|| Vec::new());
	let cl_users = users.clone();

	let users_count = use_state(|| 0 as i32);
	let cl_users_count = users_count.clone();
	let clonned_users_count = users_count.clone();

	let page = use_state(|| 1);
	let cl_page = page.clone();

	let (_, dispatch) = use_store::<Store>();
	let cl_dispatch = dispatch.clone();

	use_effect_with_deps(
		move |_| {
			wasm_bindgen_futures::spawn_local(async move {
				set_page_loading(true, cl_dispatch.clone());

				let response = api_get_users(*cl_page, 10).await;
				match response {
					Ok(users_data) => {
						set_page_loading(false, cl_dispatch.clone());
						cl_users.set(users_data.users);
						cl_users_count.set(users_data.users_count);
					}
					Err(e) => {
						set_page_loading(false, cl_dispatch.clone());
						set_show_alert(e.to_string(), cl_dispatch);
					}
				}
			});
		},
		page.clone(),
	);

	let users_list = users
		.iter()
		.enumerate()
		.map(|(index, user)| {
			html! {
				  <tr key={user.id.clone()} class={format!("bg-white border-b dark:border-gray-700 {}", if index % 2 != 0 {"dark:bg-gray-800"} else {"dark:bg-gray-900"})}>
					<th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
					  { user.name.clone() }
					</th>
					<td class="px-6 py-4">
					  {user.email.clone()}
					</td>
					<td class="px-6 py-4">
					  {user.role.clone()}
					</td>
					<td class="px-6 py-4">
					  {user.createdAt.clone()}
					</td>
					<td class="px-6 py-4">
					  {user.updatedAt.clone()}
					</td>
				  </tr>
			}
		})
		.collect::<Html>();

	html! {
	<>
	<Sidebar />
	<Section>
	<Header title="Список пользователей" />
	<ContentSection>
		{ if users.len() > 0  {
			html!{

					<div class="flex flex-col w-full gap-8 justify-between">
						<table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
							<thead class="text-xs text-gray-700 bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
							<tr>
							<th scope="col" class="px-6 py-3">
							{"Имя пользователя"}
							</th>
							<th scope="col" class="px-6 py-3">
							{"E-mail"}
							</th>
							<th scope="col" class="px-6 py-3">
							{"Роль"}
							</th>
							<th scope="col" class="px-6 py-3">
							{"Дата создания"}
							</th>
							<th scope="col" class="px-6 py-3">
							{"Дата последнего изменения"}
							</th>
							</tr>
							</thead>

							<tbody>
							{ users_list }
							</tbody>
						</table>

						<Pagination page={page} users_count={*clonned_users_count.clone()} />
					</div>

			}
		} else {
			html! {
				<div class="flex items-center justify-center">
					<Spinner />
				</div>
			}
		}}
	</ContentSection>
	</Section>
	</>
	}
}
