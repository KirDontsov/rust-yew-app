use crate::bindings;
use crate::components::{
	LineChartComponent, PieChartComponent, RadarChartComponent, Section, Sidebar,
};
use yew::prelude::*;

#[function_component(DashboardPage)]
pub fn dashboard_page() -> Html {
	html! {
	  <>
			<Sidebar />
			<Section>
				<div class="text-center py-4">
					<h1 class="text-4xl font-bold tracking-tight text-white sm:text-6xl">{"Dashboard"}</h1>
					<p class="text-lg leading-8 text-gray-300">{"Для тех, кто хочет воспользоваться преимуществами новейших возможностей искуственного интеллекта."}</p>
			  </div>
				<div style="height: calc(100vh - 188px)" class="w-full m-8 p-8 bg-white rounded-lg shadow-md dark:bg-gray-800 flex flex-col gap-8">
				<h2 class="subtitle text-white">{bindings::get_now_date()}</h2>
					<div class="flex w-full gap-8 justify-between">
						<div><LineChartComponent id="chart-1" /></div>
						<div><PieChartComponent id="chart-2" /></div>
						<div><RadarChartComponent id="chart-3" /></div>
					</div>
				</div>
			</Section>
	  </>
	}
}
