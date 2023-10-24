use crate::bindings::PieChart;
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

pub struct PieChartComponent {
	pub chart: PieChart,
	pub input_ref: NodeRef,
	pub draw_timer: Timeout,
}

impl Component for PieChartComponent {
	type Message = Msg;
	type Properties = Props;

	fn create(ctx: &Context<Self>) -> Self {
		let link = ctx.link();
		let stand_alone_timer = {
			let link = link.clone();
			Timeout::new(10, move || link.send_message(Msg::Draw))
		};
		Self {
			chart: PieChart::new(),
			draw_timer: stand_alone_timer,
			input_ref: NodeRef::default(),
		}
	}

	fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
		let id = &ctx.props().id;
		match msg {
			Msg::Draw => {
				self.chart.draw(id);
				true
			}
			Msg::DoNothing => true,
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		let id = &ctx.props().id;
		html! {
			<section class="section">
			<div class="container">
				<canvas id={id.clone()} width="500" height="500"></canvas>
			</div>
			</section>
		}
	}
}
