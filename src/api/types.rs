use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct User {
	pub id: String,
	pub name: String,
	pub email: String,
	pub role: String,
	pub photo: String,
	pub verified: bool,
	pub createdAt: DateTime<Utc>,
	pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
	pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsersData {
	pub users: Vec<User>,
	pub users_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
	pub status: String,
	pub data: UserData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsersResponse {
	pub status: String,
	pub data: UsersData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginResponse {
	pub status: String,
	pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
	pub status: String,
	pub message: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateUserSchema {
	pub name: String,
	pub email: String,
	pub role: String,
	pub verified: bool,
	// #[serde(rename = "updatedAt")]
	// pub updated_at: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct Quote {
	pub id: String,
	pub text: String,
	pub author: String,
	pub createdAt: DateTime<Utc>,
	pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuoteData {
	pub quote: Quote,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuotesData {
	pub quotes: Vec<Quote>,
	pub quotes_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuoteResponse {
	pub status: String,
	pub data: QuoteData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuotesResponse {
	pub status: String,
	pub data: QuotesData,
}
