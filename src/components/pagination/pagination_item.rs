use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
	pub item: i32,
	pub page: UseStateHandle<i32>,
}

#[function_component(PaginationItem)]
pub fn pagination_item_component(props: &Props) -> Html {
	let cl_item = props.item.clone();
	let cl_page = &props.page;

	let onclick_page = {
		let page = cl_page.clone();
		Callback::from(move |_| page.set(cl_item))
	};

	html! {
	  <>
	  <li key={cl_item} onclick={onclick_page}>
		  <a href="#" class={format!("flex items-center justify-center px-3 h-8 leading-tight text-gray-500 bg-white border border-gray-300 hover:bg-gray-100 hover:text-gray-700 dark:border-gray-700 dark:focus:bg-gray-700 dark:hover:text-white {}", if *cl_page.clone() == cl_item { "dark:bg-gray-700 dark:text-white dark:hover:bg-gray-500" } else { "dark:bg-gray-800 dark:text-gray-400 dark:hover:bg-gray-700" })}>{cl_item}</a>
	  </li>
	  </>
	}
}
