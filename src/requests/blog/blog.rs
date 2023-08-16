use reqwest::Request;
use serde::Deserialize;

use crate::{
    blog::{TumblrBlog, TumblrBlogId},
    requests::{paths, HttpMethod, TumblrRequest, TumblrRequestBuilder},
    TumblrClient,
};

#[derive(Debug)]
pub struct BlogInfoRequest {
    pub blog_id: TumblrBlogId,
}

impl TumblrRequest for BlogInfoRequest {
    type Response = BlogInfoResponse;

    fn build_request(&self, client: &TumblrClient) -> Result<Request, Box<dyn std::error::Error>> {
        Ok(TumblrRequestBuilder::new(
            &client.request_client,
            HttpMethod::Get,
            paths::blog_info(self.blog_id.clone().to_string())?,
        )?
        .auth_by_key(client.get_api_key())
        .build()?)
    }
}

#[derive(Debug, Deserialize)]
pub struct BlogInfoResponse {
    pub blog: TumblrBlog,
}
