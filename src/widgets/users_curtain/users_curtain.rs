use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::api::api_update_user_by_id;
use crate::store::{set_page_loading, set_show_alert, Store};
use crate::{
	api::{UpdateUserSchema, User},
	components::{Curtain, FormInput, LoadingButton},
};
use uuid::Uuid;
use validator::{Validate, ValidationErrors};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

fn get_input_callback(
	name: &'static str,
	cl_form: UseStateHandle<UpdateUserSchema>,
) -> Callback<String> {
	Callback::from(move |value| {
		let mut data = cl_form.deref().clone();
		match name {
			"name" => data.name = value,
			"email" => data.email = value,
			"role" => data.role = value,
			"verified" => data.verified = if value == "true" { true } else { false },
			_ => (),
		}
		cl_form.set(data);
	})
}

#[derive(PartialEq, Properties)]
pub struct Props {
	pub selected_user: UseStateHandle<User>,
	pub open: UseStateHandle<bool>,
	pub need_refetch: UseStateHandle<bool>,
}

#[function_component(UsersCurtain)]
pub fn users_curtain_component(props: &Props) -> Html {
	let open = props.open.clone();
	let need_refetch = props.need_refetch.clone();

	let selected_user = props.selected_user.clone();
	let cl_selected_user = selected_user.clone();

	let handle_close = {
		let open = open.clone();
		Callback::from(move |_| open.set(if *open { false } else { true }))
	};

	let (store, dispatch) = use_store::<Store>();
	let form = use_state(|| UpdateUserSchema::default());
	let cl_form = form.clone();

	let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

	let name_input_ref = NodeRef::default();
	let email_input_ref = NodeRef::default();
	let role_input_ref = NodeRef::default();
	let verified_input_ref = NodeRef::default();

	// устанавливаем начальное значение
	use_effect_with_deps(
		move |selected_user| {
			cl_form.set(UpdateUserSchema {
				name: selected_user.name.clone(),
				email: selected_user.email.clone(),
				role: selected_user.role.clone(),
				verified: selected_user.verified.clone(),
			})
		},
		cl_selected_user.clone(),
	);

	// синхронизация значений формы и стейта
	use_effect_with_deps(
		move |(
			cl_name_input_ref,
			cl_email_input_ref,
			cl_role_input_ref,
			cl_verified_input_ref,
			cl_form,
		)| {
			let name_input = cl_name_input_ref.cast::<HtmlInputElement>().unwrap();
			let email_input = cl_email_input_ref.cast::<HtmlInputElement>().unwrap();
			let role_input = cl_role_input_ref.cast::<HtmlInputElement>().unwrap();
			let verified_input = cl_verified_input_ref.cast::<HtmlInputElement>().unwrap();

			name_input.set_value(cl_form.name.as_str());
			email_input.set_value(cl_form.email.as_str());
			role_input.set_value(cl_form.role.as_str());
			verified_input.set_value(if cl_form.verified { "true" } else { "false" });
		},
		(
			name_input_ref.clone(),
			email_input_ref.clone(),
			role_input_ref.clone(),
			verified_input_ref.clone(),
			form.clone(),
		),
	);

	let validate_input_on_blur = {
		let cl_form = form.clone();
		let cl_validation_errors = validation_errors.clone();
		Callback::from(move |(name, value): (String, String)| {
			let mut data = cl_form.deref().clone();
			match name.as_str() {
				"email" => data.email = value,
				_ => (),
			}
			cl_form.set(data);

			match cl_form.validate() {
				Ok(_) => {
					cl_validation_errors
						.borrow_mut()
						.errors_mut()
						.remove(name.as_str());
				}
				Err(errors) => {
					cl_validation_errors
						.borrow_mut()
						.errors_mut()
						.retain(|key, _| key != &name);
					for (field_name, error) in errors.errors() {
						if field_name == &name {
							cl_validation_errors
								.borrow_mut()
								.errors_mut()
								.insert(field_name, error.clone());
						}
					}
				}
			}
		})
	};

	let handle_name_input = get_input_callback("name", form.clone());
	let handle_email_input = get_input_callback("email", form.clone());
	let handle_role_input = get_input_callback("role", form.clone());
	let handle_verified_input = get_input_callback("verified", form.clone());

	// отправка формы
	let on_submit = {
		let cl_form = form.clone();
		let cl_need_refetch = need_refetch.clone();
		let cl_validation_errors = validation_errors.clone();
		let cl_dispatch = dispatch.clone();
		let cl_open = open.clone();

		let cl_name_input_ref = name_input_ref.clone();
		let cl_email_input_ref = email_input_ref.clone();
		let cl_role_input_ref = role_input_ref.clone();
		let cl_verified_input_ref = verified_input_ref.clone();

		Callback::from(move |event: SubmitEvent| {
			let form = cl_form.clone();
			let need_refetch = cl_need_refetch.clone();
			let validation_errors = cl_validation_errors.clone();
			let dispatch = cl_dispatch.clone();
			let cl_selected_user = selected_user.clone();
			let open = cl_open.clone();

			let name_input_ref = cl_name_input_ref.clone();
			let email_input_ref = cl_email_input_ref.clone();
			let role_input_ref = cl_role_input_ref.clone();
			let verified_input_ref = cl_verified_input_ref.clone();

			event.prevent_default();
			spawn_local(async move {
				match form.validate() {
					Ok(_) => {
						let form_data = form.deref().clone();
						let form_json = serde_json::to_string(&form_data).unwrap();
						set_page_loading(true, dispatch.clone());

						let name_input = name_input_ref.cast::<HtmlInputElement>().unwrap();
						let email_input = email_input_ref.cast::<HtmlInputElement>().unwrap();
						let role_input = role_input_ref.cast::<HtmlInputElement>().unwrap();
						let verified_input = verified_input_ref.cast::<HtmlInputElement>().unwrap();

						name_input.set_value("");
						email_input.set_value("");
						role_input.set_value("");
						verified_input.set_value("");

						let res = api_update_user_by_id(
							Uuid::parse_str(cl_selected_user.id.as_str()).unwrap(),
							&form_json,
						)
						.await;

						match res {
							Ok(_) => {
								need_refetch.set(true);
								set_page_loading(false, dispatch.clone());
								set_show_alert(
									"Пользователь успешно отредактирован".to_string(),
									dispatch,
								);
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
	  <button type="button" onclick={handle_close} data-drawer-hide="drawer-right-example" aria-controls="drawer-right-example" class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 absolute top-2.5 end-2.5 inline-flex items-center justify-center dark:hover:bg-gray-600 dark:hover:text-white" >
	  <svg class="w-3 h-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 14">
	  <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"/>
	  </svg>
	  <span class="sr-only">{"Close menu"}</span>
	  </button>

		<form
			onsubmit={on_submit}
			class="w-[480px] mx-auto overflow-hidden p-8 space-y-2 text-sm flex flex-col gap-2"
		  >
			<FormInput label="Имя" name="name" input_ref={name_input_ref} handle_onchange={handle_name_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()} />
			<FormInput label="Email" name="email" input_type="email" input_ref={email_input_ref} handle_onchange={handle_email_input}  errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()} />
			<FormInput label="Роль" name="role" input_ref={role_input_ref} handle_onchange={handle_role_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()} />
			<FormInput label="Подтвержден" name="verified" input_ref={verified_input_ref} handle_onchange={handle_verified_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()} />
			<div class="grid grid-cols-2 gap-4">
			<button type="button" class="px-4 py-2 text-sm font-medium text-center text-gray-900 bg-white border border-gray-200 rounded-lg focus:outline-none hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700">{"Отмена"}</button>
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
