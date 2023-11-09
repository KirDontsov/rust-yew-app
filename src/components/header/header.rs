use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
	pub title: String,
	pub sub_title: Option<String>,
}

#[function_component(Header)]
pub fn header_component(props: &Props) -> Html {
	let cl_sub_title = props.sub_title.clone().unwrap_or_else(|| String::new());

	html! {
	  <div class="text-center py-4 flex flex-col gap-4">
		<h1 class="text-4xl font-bold tracking-tight text-white sm:text-6xl">{&props.title}</h1>
		{ if cl_sub_title.clone().len() > 0 { html! {<p class="text-lg leading-8 text-gray-300">{cl_sub_title.clone()}</p>}} else { html! {<></>}}}
	  </div>
	}
}
