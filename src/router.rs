use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
	HomePage, LoginPage, ProfilePage,
	RegisterPage, DashboardPage
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
	#[at("/")]
	HomePage,
	#[at("/register")]
	RegisterPage,
	#[at("/login")]
	LoginPage,
	#[at("/profile")]
	ProfilePage,
	#[at("/dashboard")]
	DashboardPage,
}

pub fn switch(routes: Route) -> Html {
	match routes {
		Route::HomePage => html! {<HomePage/> },
		Route::RegisterPage => html! {<RegisterPage/> },
		Route::LoginPage => html! {<LoginPage/> },
		Route::ProfilePage => html! {<ProfilePage/> },
		Route::DashboardPage => html! {<DashboardPage/> },
	}
}
