use super::pagination_item::PaginationItem;
use std::primitive::*;
use yew::prelude::*;

fn conv(x: i32) -> f32 {
	x as f32
}

pub fn generate_array(n: i32) -> Vec<i32> {
	(1..=n + 1).collect()
}

#[derive(PartialEq, Properties)]
pub struct Props {
	pub users_count: i32,
	pub page: UseStateHandle<i32>,
}

#[function_component(Pagination)]
pub fn pagination_component(props: &Props) -> Html {
	let cl_users_count = props.users_count.clone();
	let cl_page = &props.page;

	let onclick_next = {
		let page = cl_page.clone();
		Callback::from(move |_| {
			let number_of_pages = conv(cl_users_count.clone()) / 10.0;
			if *page < number_of_pages as i32 + 1 {
				page.set(*page + 1)
			}
		})
	};

	let onclick_prev = {
		let page = cl_page.clone();
		Callback::from(move |_| {
			if *page > 1 {
				page.set(*page - 1)
			} else {
				page.set(1)
			}
		})
	};

	html! {
	  <div class="flex justify-center">
			  <nav aria-label="Page navigation example">
				  <ul class="flex items-center -space-x-px h-8 text-sm">
					  <li>
						  <a href="#" onclick={onclick_prev} class="flex items-center justify-center px-3 h-8 ml-0 leading-tight text-gray-500 bg-white border border-gray-300 rounded-l-lg hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white">
							  <span class="sr-only">{"Previous"}</span>
							  <svg class="w-2.5 h-2.5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 6 10">
								  <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 1 1 5l4 4"/>
							  </svg>
						  </a>
					  </li>

					   { generate_array((conv(cl_users_count.clone()) / 10.0) as i32).iter().map(|item| {
							  html! { <PaginationItem key={*item} item={*item} page={cl_page.clone()} /> }
						  }).collect::<Html>() }

					  <li>
						  <a href="#" onclick={onclick_next} class="flex items-center justify-center px-3 h-8 leading-tight text-gray-500 bg-white border border-gray-300 rounded-r-lg hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white">
							  <span class="sr-only">{"Next"}</span>
							  <svg class="w-2.5 h-2.5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 6 10">
								  <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 9 4-4-4-4"/>
							  </svg>
						  </a>
					  </li>
				  </ul>
			  </nav>
		  </div>
	}
}
