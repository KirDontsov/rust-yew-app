use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::api::api_add_quote;
use crate::store::{set_page_loading, set_show_alert, Store};
use crate::{
	api::{AddQuoteSchema, Quote},
	components::{Curtain, FormInput, LoadingButton},
};
use validator::{Validate, ValidationErrors};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

fn get_input_callback(
	author: &'static str,
	cl_form: UseStateHandle<AddQuoteSchema>,
) -> Callback<String> {
	Callback::from(move |value| {
		let mut data = cl_form.deref().clone();
		match author {
			"author" => data.author = value,
			"text" => data.text = value,
			_ => (),
		}
		cl_form.set(data);
	})
}

#[derive(PartialEq, Properties)]
pub struct Props {
	pub selected_quote: UseStateHandle<Quote>,
	pub open: UseStateHandle<bool>,
	pub need_refetch: UseStateHandle<bool>,
}

#[function_component(QuotesCurtain)]
pub fn quotes_curtain_component(props: &Props) -> Html {
	let open = props.open.clone();
	let need_refetch = props.need_refetch.clone();

	let selected_quote = props.selected_quote.clone();

	let handle_close = {
		let open = open.clone();
		Callback::from(move |_| open.set(if *open { false } else { true }))
	};

	let (store, dispatch) = use_store::<Store>();
	let form = use_state(|| AddQuoteSchema::default());

	let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

	let author_input_ref = NodeRef::default();
	let text_input_ref = NodeRef::default();

	let validate_input_on_blur = {
		let cl_form = form.clone();
		let cl_validation_errors = validation_errors.clone();
		Callback::from(move |(author, value): (String, String)| {
			let mut data = cl_form.deref().clone();
			match author.as_str() {
				"text" => data.text = value,
				_ => (),
			}
			cl_form.set(data);

			match cl_form.validate() {
				Ok(_) => {
					cl_validation_errors
						.borrow_mut()
						.errors_mut()
						.remove(author.as_str());
				}
				Err(errors) => {
					cl_validation_errors
						.borrow_mut()
						.errors_mut()
						.retain(|key, _| key != &author);
					for (field_author, error) in errors.errors() {
						if field_author == &author {
							cl_validation_errors
								.borrow_mut()
								.errors_mut()
								.insert(field_author, error.clone());
						}
					}
				}
			}
		})
	};

	let handle_author_input = get_input_callback("author", form.clone());
	let handle_text_input = get_input_callback("text", form.clone());

	// отправка формы
	let on_submit = {
		let cl_form = form.clone();
		let cl_need_refetch = need_refetch.clone();
		let cl_validation_errors = validation_errors.clone();
		let cl_dispatch = dispatch.clone();
		let cl_open = open.clone();

		let cl_author_input_ref = author_input_ref.clone();
		let cl_text_input_ref = text_input_ref.clone();

		Callback::from(move |event: SubmitEvent| {
			let form = cl_form.clone();
			let need_refetch = cl_need_refetch.clone();
			let validation_errors = cl_validation_errors.clone();
			let dispatch = cl_dispatch.clone();
			let cl_selected_quote = selected_quote.clone();
			let open = cl_open.clone();

			let author_input_ref = cl_author_input_ref.clone();
			let text_input_ref = cl_text_input_ref.clone();

			event.prevent_default();
			spawn_local(async move {
				match form.validate() {
					Ok(_) => {
						let form_data = form.deref().clone();
						let form_json = serde_json::to_string(&form_data).unwrap();
						set_page_loading(true, dispatch.clone());

						let author_input = author_input_ref.cast::<HtmlInputElement>().unwrap();
						let text_input = text_input_ref.cast::<HtmlInputElement>().unwrap();

						author_input.set_value("");
						text_input.set_value("");

						let res = api_add_quote(&form_json).await;

						match res {
							Ok(_) => {
								need_refetch.set(true);
								set_page_loading(false, dispatch.clone());
								set_show_alert("Цитата успешно добавлена".to_string(), dispatch);
								open.set(false);
							}
							Err(e) => {
								need_refetch.set(true);
								set_page_loading(false, dispatch.clone());
								set_show_alert(e.to_string(), dispatch);
								open.set(false);
							}
						};
					}
					Err(e) => {
						validation_errors.set(Rc::new(RefCell::new(e)));
					}
				}
			});
		})
	};

	html! {
	  <Curtain open={open.clone()}>
	  <h5 id="drawer-right-label" class="inline-flex items-center mb-4 text-base font-semibold text-gray-500 dark:text-gray-400">
	  <svg class="w-4 h-4 me-2.5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 20 20">
	  <path d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5ZM9.5 4a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3ZM12 15H8a1 1 0 0 1 0-2h1v-3H8a1 1 0 0 1 0-2h2a1 1 0 0 1 1 1v4h1a1 1 0 0 1 0 2Z"/>
	  </svg>
	  { "Редактирование пользователя" }</h5>
	  <button type="button" onclick={handle_close.clone()} data-drawer-hide="drawer-right-example" aria-controls="drawer-right-example" class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 absolute top-2.5 end-2.5 inline-flex items-center justify-center dark:hover:bg-gray-600 dark:hover:text-white" >
	  <svg class="w-3 h-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 14">
	  <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"/>
	  </svg>
	  <span class="sr-only">{"Close menu"}</span>
	  </button>

		<form
			onsubmit={on_submit}
			class="w-[480px] mx-auto overflow-hidden p-8 space-y-2 text-sm flex flex-col gap-2"
		  >
			<FormInput label="Автор" name="author" input_ref={author_input_ref} handle_onchange={handle_author_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()} />
			<FormInput label="Текст" name="text" input_type="text" input_ref={text_input_ref} handle_onchange={handle_text_input}  errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()} />
			<div class="grid grid-cols-2 gap-4">
			<button onclick={handle_close.clone()} type="button" class="px-4 py-2 text-sm font-medium text-center text-gray-900 bg-white border border-gray-200 rounded-lg focus:outline-none hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700">{"Отмена"}</button>
			<LoadingButton
					loading={store.page_loading}
				>
				{"Сохранить"}
				</LoadingButton>
			</div>
		</form>
	  </Curtain>
	}
}
