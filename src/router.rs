use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
	AccountPage, DashboardPage, FirmsInfoPage, FirmsPage, FirmsReviewsPage, HomePage, LoginPage,
	MapsPage, NotFoundPage, QuotesPage, RegisterPage, UsersPage,
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
	#[at("/users")]
	UsersPage,
	#[at("/quotes")]
	QuotesPage,
	#[at("/maps")]
	MapsPage,
	#[at("/crawler/firms")]
	FirmsPage,
	#[at("/crawler/firms_info")]
	FirmsInfoPage,
	#[at("/crawler/firms_reviews")]
	FirmsReviewsPage,
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
		Route::UsersPage => html! {<UsersPage/> },
		Route::QuotesPage => html! {<QuotesPage/> },
		Route::MapsPage => html! { <MapsPage /> },
		Route::FirmsPage => html! {<FirmsPage/> },
		Route::FirmsInfoPage => html! { <FirmsInfoPage /> },
		Route::FirmsReviewsPage => html! { <FirmsReviewsPage /> },
		Route::NotFound => html! { <NotFoundPage /> },
	}
}
