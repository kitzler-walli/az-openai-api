use serde::{Deserialize, Serialize};
use ureq::{Agent, AgentBuilder};

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
	pub api_key: String,
}

impl Clone for Auth {
	fn clone(&self) -> Self {
		Self { api_key: self.api_key.clone() }
	}
}

#[allow(dead_code)]
impl Auth {
	pub fn new(api_key: &str) -> Auth {
		Auth { api_key: api_key.to_string() }
	}

	pub fn from_env() -> Result<Self, String> {
		let api_key =
			std::env::var("API_KEY").map_err(|_| "Missing API_KEY".to_string())?;
		Ok(Self { api_key })
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ApiType {
	Audio,
	Chat,
	Completions,
	Images,
}

impl ApiType {
	pub fn from_env() -> Result<Self, String> {
		match std::env::var("API_TYPE").as_deref() {
			Ok("audio") => Ok(ApiType::Audio),
			Ok("chat") => Ok(ApiType::Chat),
			Ok("completions") => Ok(ApiType::Completions),
			Ok("images") => Ok(ApiType::Images),
			_ => Err("Invalid or missing API_TYPE".to_string()),
		}
	}
}

#[derive(Debug)]
pub struct OpenAI {
	pub auth: Auth,
	pub api_url: String,
	pub api_type: ApiType,
	pub(crate) agent: Agent,
}

impl Clone for OpenAI {
	fn clone(&self) -> Self {
		Self { auth: self.auth.clone(), api_url: self.api_url.clone(), api_type: self.api_type, agent: self.agent.clone() }
	}
}

#[allow(dead_code)]
impl OpenAI {
	pub fn new(auth: Auth, api_url: &str, api_type: ApiType) -> OpenAI {
		OpenAI { auth, api_url: api_url.to_string(), api_type, agent: AgentBuilder::new().build() }
	}

	pub fn set_proxy(mut self, proxy: &str) -> OpenAI {
		let proxy = ureq::Proxy::new(proxy).unwrap();
		self.agent = ureq::AgentBuilder::new().proxy(proxy).build();
		self
	}

	pub fn use_env_proxy(mut self) -> OpenAI {
		let proxy = match (std::env::var("http_proxy"), std::env::var("https_proxy")) {
			(Ok(http_proxy), _) => Some(http_proxy),
			(_, Ok(https_proxy)) => Some(https_proxy),
			_ => {
				log::warn!("Missing http_proxy or https_proxy");
				None
			},
		};
		if let Some(proxy) = proxy {
			let proxy = ureq::Proxy::new(&proxy).unwrap();
			self.agent = ureq::AgentBuilder::new().proxy(proxy).build();
		}
		self
	}
}

#[cfg(test)]
pub fn new_test_openai() -> OpenAI {
	let auth = Auth::from_env().unwrap();
	let api_type = ApiType::from_env().unwrap_or(ApiType::Chat); // Default to Chat if not specified
	OpenAI::new(auth, "https://kwnet-openai.openai.azure.com/openai/deployments/gpt-4o/chat/completions?api-version=2024-02-15-preview", api_type).use_env_proxy()
}