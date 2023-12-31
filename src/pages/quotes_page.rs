use crate::{
	api::{api_get_quotes, Quote},
	components::{ContentSection, Header, Pagination, Section, Sidebar, Spinner},
	store::{set_page_loading, set_show_alert, Store},
	widgets::QuotesCurtain,
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(QuotesPage)]
pub fn quotes_page() -> Html {
	let quotes = use_state(|| Vec::new());
	let cl_quotes = quotes.clone();

	let need_refetch = use_state(|| false);

	let selected_quote = use_state(|| Quote::default());
	let cl_selected_quote = selected_quote.clone();

	let open = use_state(|| false);
	let cl_open = open.clone();

	let quotes_count = use_state(|| 0);
	let cl_quotes_count = quotes_count.clone();
	let clonned_quotes_count = quotes_count.clone();

	let page = use_state(|| 1);
	let cl_page = page.clone();

	let (_, dispatch) = use_store::<Store>();
	let cl_dispatch = dispatch.clone();

	// фетч при инициализации и изменении пагинации
	use_effect_with_deps(
		move |_| {
			wasm_bindgen_futures::spawn_local(async move {
				set_page_loading(true, cl_dispatch.clone());

				let response = api_get_quotes(*cl_page, 10).await;
				match response {
					Ok(quotes_data) => {
						set_page_loading(false, cl_dispatch.clone());
						cl_quotes.set(quotes_data.quotes);
						cl_quotes_count.set(quotes_data.quotes_count);
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

	let handle_add = {
		let cl_open = open.clone();
		Callback::from(move |_| {
			cl_open.set(true);
		})
	};

	let quotes_list = quotes
		.iter()
		.enumerate()
		.map(|(index, quote)| {

				let handle_select = {
					let cl_quote = quote.clone();
					let cl_selected_quote = selected_quote.clone();
					let clo_open = open.clone();
					Callback::from(move |_| {
						cl_selected_quote.set(cl_quote.clone());
						clo_open.set(true);
					})
				};

			html! {
				  <tr key={quote.id.clone()} onclick={handle_select.clone()} class={format!("bg-white border-b dark:border-gray-700 {}", if index % 2 != 0 {"dark:bg-gray-800"} else {"dark:bg-gray-900"})}>
					<th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
					  { quote.author.clone() }
					</th>
					<td class="px-6 py-4">
					  {quote.text.clone()}
					</td>
					<td class="px-6 py-4 min-w-[250px]">
					  {quote.createdAt.clone()}
					</td>
					<td class="px-6 py-4 min-w-[250px]">
					  {quote.updatedAt.clone()}
					</td>
				  </tr>
			}
		})
		.collect::<Html>();

	html! {
	<>
	<Sidebar />
	<Section>
	<Header title="Список цитат" />
	<ContentSection>
	  <div>
	  <button onclick={handle_add.clone()} type="button" class="px-4 py-2 text-sm font-medium text-center text-gray-900 bg-white border border-gray-200 rounded-lg focus:outline-none hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700">
	  <svg class="w-6 h-6 text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 18 18">
	  <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 1v16M1 9h16"/>
	  </svg>
	  </button>
		</div>
		{ if quotes.len() > 0  {
			html!{

					<div class="flex flex-col w-full gap-8 justify-between">
						<table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
							<thead class="text-xs text-gray-700 bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
							<tr>
							<th scope="col" class="px-6 py-3">
							{"Имя автора"}
							</th>
							<th scope="col" class="px-6 py-3">
							{"Цитата"}
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
							{ quotes_list }
							</tbody>
						</table>

						<Pagination page={page} count={*clonned_quotes_count.clone()} />
					<QuotesCurtain open={cl_open.clone()} selected_quote={cl_selected_quote.clone()} need_refetch={need_refetch.clone()} />
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
