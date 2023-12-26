use super::types::{ErrorResponse, User, UserLoginResponse, UserResponse};
use reqwasm::http;

pub async fn api_start_parse() -> Result<(), String> {
	let response = match http::Request::get("http://localhost:8000/api/crawler/firms")
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

	// let res_json = response.json::<UserResponse>().await;
	// match res_json {
	// 	Ok(data) => Ok(data.data.user),
	// 	Err(_) => Err("Не удалось прочитать ответ".to_string()),
	// }
}

pub async fn api_parse_firms_info() -> Result<(), String> {
	let response = match http::Request::get("http://localhost:8000/api/crawler/firms_info")
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

	// let res_json = response.json::<UserResponse>().await;
	// match res_json {
	// 	Ok(data) => Ok(data.data.user),
	// 	Err(_) => Err("Не удалось прочитать ответ".to_string()),
	// }
}
