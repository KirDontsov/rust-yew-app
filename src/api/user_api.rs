use super::types::{ErrorResponse, User, UserLoginResponse, UserResponse, UsersResponse};
use reqwasm::http;

pub async fn api_get_user_by_id(id: uuid::Uuid) -> Result<User, String> {
	let response =
		match http::Request::get(&format!("http://localhost:8000/api/user/{}", &id).to_string())
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

pub async fn api_get_users() -> Result<Vec<User>, String> {
	let response = match http::Request::get("http://localhost:8000/api/users")
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

	let res_json = response.json::<UsersResponse>().await;
	match res_json {
		Ok(data) => Ok(data.data.users),
		Err(_) => Err("Не удалось прочитать ответ".to_string()),
	}
}
