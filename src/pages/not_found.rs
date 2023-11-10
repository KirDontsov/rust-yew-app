use crate::components::{Header, Section, Sidebar};
use yew::prelude::*;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
	html! {
	<>
	  <Sidebar />
	  <Section>
	  <div class="flex w-full items-center justify-center">
		<Header title="404" sub_title="Страница не найдена" />
	  </div>
	  </Section>
	</>
	}
}
