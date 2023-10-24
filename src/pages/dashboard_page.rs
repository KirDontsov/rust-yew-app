use crate::components::{Section, Sidebar};
use yew::prelude::*;

#[function_component(DashboardPage)]
pub fn dashboard_page() -> Html {
	html! {
	  <>
			<Sidebar />
			<Section>
				<div class="text-center">
					<h1 class="text-4xl font-bold tracking-tight text-white sm:text-6xl">{"Dashboard"}</h1>
					<p class="mt-6 text-lg leading-8 text-gray-300">{"Для тех, кто хочет воспользоваться преимуществами новейших возможностей искуственного интеллекта."}</p>
			  </div>
			</Section>
	  </>
	}
}
