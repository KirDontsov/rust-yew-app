use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/js_time.js")]
extern "C" {
	#[wasm_bindgen]
	pub fn get_now_date() -> String;
}

#[wasm_bindgen(module = "/js/line_chart.js")]
extern "C" {
	pub type LineChart;

	#[wasm_bindgen(constructor)]
	pub fn new() -> LineChart;

	#[wasm_bindgen(method)]
	pub fn draw(this: &LineChart, element_id: &str);

	#[wasm_bindgen(method)]
	pub fn update(this: &LineChart, value: i32);
}

#[wasm_bindgen(module = "/js/pie_chart.js")]
extern "C" {
	pub type PieChart;

	#[wasm_bindgen(constructor)]
	pub fn new() -> PieChart;

	#[wasm_bindgen(method)]
	pub fn draw(this: &PieChart, element_id: &str);

	#[wasm_bindgen(method)]
	pub fn update(this: &PieChart, value: i32);
}

#[wasm_bindgen(module = "/js/radar_chart.js")]
extern "C" {
	pub type RadarChart;

	#[wasm_bindgen(constructor)]
	pub fn new() -> RadarChart;

	#[wasm_bindgen(method)]
	pub fn draw(this: &RadarChart, element_id: &str);

	#[wasm_bindgen(method)]
	pub fn update(this: &RadarChart, value: i32);
}

#[wasm_bindgen(module = "/js/maps.js")]
extern "C" {
	pub type Maps;

	#[wasm_bindgen(constructor)]
	pub fn new() -> Maps;

	#[wasm_bindgen(method)]
	pub fn draw(this: &Maps, element_id: &str);

	#[wasm_bindgen(method)]
	pub fn update(this: &Maps, value: i32);
}
