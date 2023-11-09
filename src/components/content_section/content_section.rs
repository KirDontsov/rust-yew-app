use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
	pub h: Option<String>,
	pub children: Children,
}

#[function_component(ContentSection)]
pub fn content_section_component(props: &Props) -> Html {
	let h = props.h.clone().unwrap_or_else(|| "156".to_string());

	html! {
	<div style={format!("height: calc(100vh - {}px)", h)} class="w-full m-8 p-8 bg-white rounded-lg shadow-md dark:bg-gray-800 flex flex-col gap-8">
	  {props.children.clone()}
	</div>
	}
}
