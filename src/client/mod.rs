#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use serde::{
    de::{value, DeserializeOwned},
    Serialize,
};
use serde_json::{json, Deserializer, Value};

use crate::{
    completions::{CompletionRequest, CompletionResponse},
    edits::{EditRequest, EditResponse},
    error::OpenAIError,
    images::{CreateImageRequest, ImageResponse},
    models::{CompletionModel, EditModel},
};

pub trait OpenAIRequest {
    fn endpoint(&self) -> &str;
}

pub trait OpenAIResponse {}

pub struct OpenAIClient {
    client: reqwest::Client,
    api_key: String,
}

impl OpenAIClient {
    pub fn new(api_key: String) -> Self {
        OpenAIClient {
            client: reqwest::Client::new(),
            api_key,
        }
    }

    pub async fn list_models(&self) -> Result<serde_json::Value, reqwest::Error> {
        let request = self
            .client
            .get("https://api.openai.com/v1/models")
            .bearer_auth(&self.api_key);

        let resp = request.send().await?;
        resp.json::<serde_json::Value>().await
    }

    pub async fn complete(
        &self,
        model: &CompletionModel,
        prompt: &str,
    ) -> Result<CompletionResponse, OpenAIError> {
        //TODO: Add error handling for when the model max tokens < prompt length
        self.send_request::<CompletionRequest, CompletionResponse>(
            CompletionRequest::new(model.name, prompt).max_tokens(model.max_tokens - prompt.len()),
        )
        .await
    }

    pub async fn edit(
        &self,
        model: &EditModel,
        input: &str,
        instruction: &str,
    ) -> Result<EditResponse, OpenAIError> {
        dbg!(json!(EditRequest::new(model.name, instruction).input(input)));
        self.send_request::<EditRequest, EditResponse>(
            EditRequest::new(model.name, instruction).input(input),
        )
        .await
    }

    pub async fn create_image(&self, prompt: &str) -> Result<ImageResponse, OpenAIError> {
        self.send_request::<CreateImageRequest, ImageResponse>(CreateImageRequest::new(prompt))
            .await
    }

    pub async fn send_request<
        Req: OpenAIRequest + Serialize,
        Res: OpenAIResponse + DeserializeOwned,
    >(
        &self,
        request: Req,
    ) -> Result<Res, OpenAIError> {
        let body = json!(request);

        let request_builder = self
            .client
            .post(request.endpoint())
            .json(&body)
            .bearer_auth(&self.api_key);

        let response = request_builder.send().await?.json::<Value>().await?;

        if response.get("error").is_some() {
            let err_json = json!(response.get("error").unwrap());
            
            panic!("{}", err_json.to_string());

            let not_active = "billing_not_active".to_string();
            let err_json = json!(response.get("error").unwrap());

            match err_json
                .get("type")
                .expect("No 'type' sent with error message")
                .as_str()
                .unwrap()
            {
                //not_active => return Err(OpenAIError::BillingNotActive),
                other => return Err(OpenAIError::UnrecognizedError(other)),
            }
        } else {
            //Handle the error
            Ok(Res::deserialize(response).expect("need to handle this error"))
        }
    }
}



pub struct OpenAIRequestBuilder {}
