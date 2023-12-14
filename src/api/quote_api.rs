use super::types::{ErrorResponse, Quote, QuoteResponse, QuotesData, QuotesResponse};
use reqwasm::http;

pub async fn api_get_quote_by_id(id: uuid::Uuid) -> Result<Quote, String> {
	let response =
		match http::Request::get(&format!("http://localhost:8000/api/quote/{}", &id).to_string())
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

	let res_json = response.json::<QuoteResponse>().await;
	match res_json {
		Ok(data) => Ok(data.data.quote),
		Err(_) => Err("Не удалось прочитать ответ".to_string()),
	}
}

pub async fn api_get_quotes(page: i32, limit: i32) -> Result<QuotesData, String> {
	let response = match http::Request::get(&format!(
		"http://localhost:8000/api/quotes?page={}&limit={}",
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

	let res_json = response.json::<QuotesResponse>().await;
	match res_json {
		Ok(data) => Ok(data.data),
		Err(_) => Err("Не удалось прочитать ответ".to_string()),
	}
}
