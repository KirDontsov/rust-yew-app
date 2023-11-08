use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
	pub active: bool,
	pub to: Route,
	pub children: Children,
}

#[function_component(SidebarItem)]
pub fn sidebar_item_component(props: &Props) -> Html {
	let active = props.active.clone();
	let to = props.to.clone();

	html! {
		<li>
			<Link<Route> to={to}>
				<span class={format!("flex items-center p-2 rounded-lg hover:bg-gray-700 group {}", if active { "text-indigo-400 hover:text-indigo-300" } else { "text-gray-400 hover:text-white" })}>{props.children.clone()}</span>
			</Link<Route>>
	 </li>
	}
}
