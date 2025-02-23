use crate::mpart::Mpart as Multipart;

use crate::openai::OpenAI;
use crate::*;

#[cfg(not(test))]
use log::{debug, error, info};

#[cfg(test)]
use std::{eprintln as error, println as info, println as debug};

pub trait Requests {
	fn post(&self, body: Json) -> ApiResult<Json>;
	fn post_multipart(&self, multipart: Multipart) -> ApiResult<Json>;
	fn get(&self) -> ApiResult<Json>;
}

impl Requests for OpenAI {
	fn post(&self, body: Json) -> ApiResult<Json> {
		info!("===> 🚀\n\tPost api, body: {body}");

		let response = self
			.agent
			.post(&self.api_url)
			.set("Content-Type", "application/json")
			.set("api-key", &self.auth.api_key)
			.send_json(body);

		deal_response(response)
	}

	fn get(&self) -> ApiResult<Json> {
		info!("===> 🚀\n\tGet api");

		let response = self
			.agent
			.get(&self.api_url)
			.set("Content-Type", "application/json")
			.set("api-key", &self.auth.api_key)
			.call();

		deal_response(response)
	}

	fn post_multipart(&self, mut multipart: Multipart) -> ApiResult<Json> {
		info!("===> 🚀\n\tPost multipart api, multipart: {:?}", multipart);

		let form_data = multipart.prepare().unwrap();

		let response = self
			.agent
			.post(&self.api_url)
			.set("Content-Type", &format!("multipart/form-data; boundary={}", form_data.boundary()))
			.set("api-key", &self.auth.api_key)
			.send(form_data);

		deal_response(response)
	}
}

fn deal_response(response: Result<ureq::Response, ureq::Error>) -> ApiResult<Json> {
	match response {
		Ok(resp) => {
			let json = resp.into_json::<Json>().unwrap();
			debug!("<== ✔️\n\tDone api, resp: {json}");
			return Ok(json);
		},
		Err(err) => match err {
			ureq::Error::Status(status, response) => {
				let error_msg = response.into_json::<Json>().unwrap();
				error!("<== ❌\n\tError api, status: {status}, error: {error_msg}");
				return Err(Error::ApiError(format!("{error_msg}")));
			},
			ureq::Error::Transport(e) => {
				error!("<== ❌\n\tError api, error: {:?}", e.to_string());
				return Err(Error::RequestError(e.to_string()));
			},
		},
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::openai;
	use ureq::json;

	#[test]
	fn test_post() {
		let openai = openai::new_test_openai();
		let body = json!({
			"messages": [{"role": "user", "content": "Say this is a test!"}],
			"temperature": 0.7
		});
		let result = openai.post(body).unwrap();
		assert!(result.to_string().contains("This is a test"));
	}

	#[test]
	fn test_get() {
		let openai = openai::new_test_openai();
		let resp = openai.get().unwrap();
		assert!(resp.to_string().contains("babbage-002"));
	}
}
