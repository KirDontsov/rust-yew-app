use crate::app::App;

mod api;
mod app;
mod components;
mod pages;
mod router;
mod store;

fn main() {
	yew::Renderer::<App>::new().render();
}