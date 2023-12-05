use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
	pub children: Children,
	pub open: UseStateHandle<bool>,
}

#[function_component(Curtain)]
pub fn curtain_component(props: &Props) -> Html {
	html! {
	  <div id="drawer-right-example" class={format!("fixed top-0 z-40 h-screen p-4 overflow-y-auto transition-transform translate-x-full bg-white w-[40rem] dark:bg-gray-800 {}", if *props.open.clone() { "right-[40rem]" } else { "right-0" })} tabindex="-1" aria-labelledby="drawer-right-label">
		{props.children.clone()}
	  </div>
	}
}
