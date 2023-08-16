use serde::{Deserialize, Serialize};

use crate::{
    blog::TumblrBlogId,
    post::{PostCreate, PostGet},
    requests::{paths, HttpMethod, TumblrRequest, TumblrRequestBuilder},
    TumblrClient,
};

#[derive(Debug)]
pub struct PostCreateRequest {
    pub blog_id: TumblrBlogId,
    pub parameters: PostCreate,
}

impl TumblrRequest for PostCreateRequest {
    type Response = PostCreateResponse;

    fn build_request(
        &self,
        client: &TumblrClient,
    ) -> Result<reqwest::Request, Box<dyn std::error::Error>> {
        Ok(TumblrRequestBuilder::new(
            &client.request_client,
            HttpMethod::Post,
            paths::blog_post_create(self.blog_id.clone().to_string())?,
        )?
        .auth_by_oauth(client.get_access_token())
        .json(serde_json::to_string(&self.parameters)?)
        .build()?)
    }
}

#[derive(Debug, Deserialize)]
pub struct PostCreateResponse {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct PostGetRequest {
    pub blog_id: TumblrBlogId,
    pub post_id: String,
}

impl TumblrRequest for PostGetRequest {
    type Response = PostGetResponse;

    fn build_request(
        &self,
        client: &TumblrClient,
    ) -> Result<reqwest::Request, Box<dyn std::error::Error>> {
        Ok(TumblrRequestBuilder::new(
            &client.request_client,
            HttpMethod::Get,
            paths::blog_post(self.blog_id.clone().to_string(), self.post_id.clone())?,
        )?
        .auth_by_oauth(client.get_access_token())
        .build()?)
    }
}

#[derive(Debug, Deserialize)]
pub struct PostGetResponse {
    #[serde(flatten)]
    pub parameters: PostGet,
}
