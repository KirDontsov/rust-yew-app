use chrono::{Datelike, Local};
use gloo_console::log;
use wasm_bindgen::JsValue;
use yew::prelude::*;

// #[derive(PartialEq, Properties)]
// pub struct Props {
// 	pub children: Children,
// 	pub open: UseStateHandle<bool>,
// }

const REFORM_YEAR: u32 = 1099;

fn is_leap_year(year: u32) -> bool {
	if year <= REFORM_YEAR {
		return year % 4 == 0;
	}
	(year % 4 == 0) ^ (year % 100 == 0) ^ (year % 400 == 0)
}

fn days_by_month(year: u32) -> Vec<u32> {
	let feb_day: u32 = if is_leap_year(year) { 29 } else { 28 };
	vec![0, 31, feb_day, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
}

fn get_days_accumulated_by_month(year: u32) -> Vec<u32> {
	let days: Vec<u32> = days_by_month(year);
	days
}

fn default_year() -> u32 {
	let now = Local::now();
	let (_, year) = now.year_ce();
	year
}

fn current_month() -> u32 {
	let now = Local::now();
	let month = now.month();
	month
}

#[function_component(CalendarComponent)]
pub fn calendar_component() -> Html {
	let days_in_months = get_days_accumulated_by_month(default_year());
	let month = current_month();
	let days_count = days_in_months[month as usize];

	let calendar = (1..=days_count)
		.map(|day| {
			let day_from_map = JsValue::from(format!("{:?}", &day));
			log!("day_from_map", day_from_map);
			html! {
      <button type="button" class="flex flex-col p-6 bg-white border border-gray-200 rounded-lg shadow hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 dark:hover:bg-gray-700 w-[70px]">
        <h5 class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{day.clone()}</h5>
      </button>
			}
		})
		.collect::<Html>();

	html! {
	  <div id="drawer-right-example" class={format!("text-white {}", "")} tabindex="-1" aria-labelledby="drawer-right-label">
	  <h5 class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{"Декабрь"}</h5>

	  <div class="flex gap-2 w-[550px] flex-wrap mt-2">
	  {calendar}
	  </div>
	  </div>
	}
}
