use super::types::{ErrorResponse, User, UserLoginResponse, UserResponse};
use reqwasm::http;

pub async fn api_register_user(user_data: &str) -> Result<User, String> {
	let response = match http::Request::post("http://localhost:8000/api/auth/register")
		.header("Content-Type", "application/json")
		.body(user_data)
		.send()
		.await
	{
		Ok(res) => res,
		Err(_) => return Err("Не удалось сделать запрос".to_string()),
	};

	if response.status() != 200 {
		let error_response = response.json::<ErrorResponse>().await;
		return if let Ok(error_response) = error_response {
			Err(error_response.message)
		} else {
			Err(format!("Ошибка API: {}", response.status()))
		};
	}

	let res_json = response.json::<UserResponse>().await;
	match res_json {
		Ok(data) => Ok(data.data.user),
		Err(_) => Err("Не удалось прочитать ответ".to_string()),
	}
}

pub async fn api_login_user(credentials: &str) -> Result<UserLoginResponse, String> {
	let response = match http::Request::post("http://localhost:8000/api/auth/login")
		.header("Content-Type", "application/json")
		.credentials(http::RequestCredentials::Include)
		.body(credentials)
		.send()
		.await
	{
		Ok(res) => res,
		Err(_) => return Err("Не удалось сделать запрос".to_string()),
	};

	if response.status() != 200 {
		let error_response = response.json::<ErrorResponse>().await;
		return if let Ok(error_response) = error_response {
			Err(error_response.message)
		} else {
			Err(format!("Ошибка API: {}", response.status()))
		};
	}

	let res_json = response.json::<UserLoginResponse>().await;
	match res_json {
		Ok(data) => Ok(data),
		Err(_) => Err("Не удалось прочитать ответ".to_string()),
	}
}

pub async fn api_user_info() -> Result<User, String> {
	let response = match http::Request::get("http://localhost:8000/api/users/me")
		.credentials(http::RequestCredentials::Include)
		.send()
		.await
	{
		Ok(res) => res,
		Err(_) => return Err("Не удалось сделать запрос".to_string()),
	};

	if response.status() != 200 {
		let error_response = response.json::<ErrorResponse>().await;
		return if let Ok(error_response) = error_response {
			Err(error_response.message)
		} else {
			Err(format!("Ошибка API: {}", response.status()))
		};
	}

	let res_json = response.json::<UserResponse>().await;
	match res_json {
		Ok(data) => Ok(data.data.user),
		Err(_) => Err("Не удалось прочитать ответ".to_string()),
	}
}

pub async fn api_logout_user() -> Result<(), String> {
	let response = match http::Request::get("http://localhost:8000/api/auth/logout")
		.credentials(http::RequestCredentials::Include)
		.send()
		.await
	{
		Ok(res) => res,
		Err(_) => return Err("Не удалось сделать запрос".to_string()),
	};

	if response.status() != 200 {
		let error_response = response.json::<ErrorResponse>().await;
		return if let Ok(error_response) = error_response {
			Err(error_response.message)
		} else {
			Err(format!("Ошибка API: {}", response.status()))
		};
	}

	Ok(())
}
