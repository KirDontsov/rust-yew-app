use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
	AccountPage, DashboardPage, HomePage, LoginPage, MapsPage, NotFoundPage, ParserPage,
	RegisterPage, UsersPage,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
	#[at("/")]
	HomePage,
	#[at("/register")]
	RegisterPage,
	#[at("/login")]
	LoginPage,
	#[at("/account")]
	AccountPage,
	#[at("/dashboard")]
	DashboardPage,
	#[at("/parser")]
	ParserPage,
	#[at("/users")]
	UsersPage,
	#[at("/maps")]
	MapsPage,
	#[not_found]
	#[at("/404")]
	NotFound,
}

pub fn switch(routes: Route) -> Html {
	match routes {
		Route::HomePage => html! {<HomePage/> },
		Route::RegisterPage => html! {<RegisterPage/> },
		Route::LoginPage => html! {<LoginPage/> },
		Route::AccountPage => html! {<AccountPage/> },
		Route::DashboardPage => html! {<DashboardPage/> },
		Route::ParserPage => html! {<ParserPage/> },
		Route::UsersPage => html! {<UsersPage/> },
		Route::MapsPage => html! { <MapsPage /> },
		Route::NotFound => html! { <NotFoundPage /> },
	}
}
