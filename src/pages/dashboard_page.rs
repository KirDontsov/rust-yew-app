use crate::bindings;
use crate::components::{
	ContentSection, Header, LineChartComponent, PieChartComponent, RadarChartComponent, Section,
	Sidebar,
};
use yew::prelude::*;

#[function_component(DashboardPage)]
pub fn dashboard_page() -> Html {
	html! {
	  <>
			<Sidebar />
			<Section>
			<Header title="С возвращением!" />
			<ContentSection>
				<h2 class="subtitle text-white">{bindings::get_now_date()}</h2>
				<div class="flex w-full gap-8 justify-between">
					<div><LineChartComponent id="chart-1" /></div>
					<div><PieChartComponent id="chart-2" /></div>
					<div><RadarChartComponent id="chart-3" /></div>
				</div>
			</ContentSection>
			</Section>
	  </>
	}
}
