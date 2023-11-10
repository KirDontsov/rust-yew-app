use crate::components::{ContentSection, Header, MapsComponent, Section, Sidebar};
use yew::prelude::*;

#[function_component(MapsPage)]
pub fn maps_page() -> Html {
	html! {
	  <>
			<Sidebar />
			<Section>
			<Header title="Карта" />
			<ContentSection>
				<MapsComponent id="map"/>
			</ContentSection>
			</Section>
	  </>
	}
}
