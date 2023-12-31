use super::types::{ErrorResponse, User, UserData, UserResponse, UsersData, UsersResponse};
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

pub async fn api_get_users(page: i32, limit: i32) -> Result<UsersData, String> {
	let response = match http::Request::get(&format!(
		"http://localhost:8000/api/users?page={}&limit={}",
		page, limit
	))
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
		} else if response.status() == 403 {
			Err(format!("Доступ запрещен {}", response.status()))
		} else {
			Err(format!("Ошибка API: {}", response.status()))
		};
	}

	let res_json = response.json::<UsersResponse>().await;
	match res_json {
		Ok(data) => Ok(data.data),
		Err(_) => Err("Не удалось прочитать ответ".to_string()),
	}
}

pub async fn api_update_user_by_id(id: uuid::Uuid, user_data: &str) -> Result<UserData, String> {
	let response =
		match http::Request::put(format!("http://localhost:8000/api/user/{}", id).as_str())
			.header("Content-Type", "application/json")
			.credentials(http::RequestCredentials::Include)
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
		} else if response.status() == 403 {
			Err(format!("Доступ запрещен {}", response.status()))
		} else {
			Err(format!("Ошибка API: {}", response.status()))
		};
	}

	let res_json = response.json::<UserResponse>().await;
	match res_json {
		Ok(data) => Ok(data.data),
		Err(_) => Err("Не удалось прочитать ответ".to_string()),
	}
}
