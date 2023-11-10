use crate::bindings::Maps;
use gloo::timers::callback::Timeout;
use yew::prelude::*;

pub enum Msg {
	Draw,
	DoNothing,
}

#[derive(Properties, PartialEq)]
pub struct Props {
	pub id: String,
}

pub struct MapsComponent {
	pub maps: Maps,
	pub input_ref: NodeRef,
	pub draw_maps: Timeout,
}

impl Component for MapsComponent {
	type Message = Msg;
	type Properties = Props;

	fn create(ctx: &Context<Self>) -> Self {
		let link = ctx.link();
		let stand_alone_timer = {
			let link = link.clone();
			Timeout::new(10, move || link.send_message(Msg::Draw))
		};
		Self {
			maps: Maps::new(),
			draw_maps: stand_alone_timer,
			input_ref: NodeRef::default(),
		}
	}

	fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
		let id = &ctx.props().id;
		match msg {
			Msg::Draw => {
				self.maps.draw(id);
				true
			}
			Msg::DoNothing => true,
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		let id = &ctx.props().id;
		html! {
			<div id={id.clone()} style="width: 100%; height: 100%"></div>
		}
	}
}
