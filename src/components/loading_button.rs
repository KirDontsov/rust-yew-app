use super::spinner::Spinner;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
	pub loading: bool,
	pub btn_color: Option<String>,
	pub text_color: Option<String>,
	pub children: Children,
}

#[function_component(LoadingButton)]
pub fn loading_button_component(props: &Props) -> Html {
	let text_color = props
		.text_color
		.clone()
		.unwrap_or_else(|| "text-white".to_string());
	let btn_color = props
		.btn_color
		.clone()
		.unwrap_or_else(|| "bg-indigo-600".to_string());

	html! {
	<button
	  type="submit"
	  class={format!(
		"hover:bg-indigo-500 w-full py-2 font-semibold outline-none border-none flex justify-center rounded-md shadow-sm focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 {}",
		 if props.loading {"bg-[#ccc]"} else {btn_color.as_str()}
	  )}
	>
	  if props.loading {
		<div class="flex items-center justify-center">
		  <Spinner />
		</div>
	  }else{
		<span class={text_color.to_owned()}>{props.children.clone()}</span>
	  }
	</button>
	}
}
