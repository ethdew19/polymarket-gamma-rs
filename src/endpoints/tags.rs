use crate::{
    GammaClient, RestError,
    types::types_tags::{
        GetRelatedTagByIdArgs, GetRelatedTagByIdResponse, GetRelatedTagBySlugArgs,
        GetRelatedTagBySlugResponse, GetTagByIdArgs, GetTagByIdResponse, GetTagBySlugArgs,
        GetTagBySlugResponse, GetTagsRelatedToIdArgs, GetTagsRelatedToIdResponse,
        GetTagsRelatedToSlugArgs, GetTagsRelatedToSlugResponse, ListTagsArgs, ListTagsResponse,
    },
};

impl GammaClient {
    pub async fn list_tags(&self, args: ListTagsArgs) -> Result<ListTagsResponse, RestError> {
        let path = format!("{}/tags", self.base_url);

        let response = self
            .http_client
            .get(path)
            .query(&args)
            .send()
            .await
            .map_err(RestError::RequestError)?;

        let status = response.status();
        let raw_text = response.text().await.map_err(RestError::RequestError)?;

        if !status.is_success() {
            return Err(RestError::HttpError {
                status,
                body: raw_text,
            });
        }

        serde_json::from_str(&raw_text).map_err(|e| RestError::ParseError {
            error: e,
            raw_json: raw_text,
        })
    }
    pub async fn get_tag_by_id(
        &self,
        args: GetTagByIdArgs,
    ) -> Result<GetTagByIdResponse, RestError> {
        let path = format!("{}/tags/{}", self.base_url, args.id);

        let response = self
            .http_client
            .get(path)
            .query(&args)
            .send()
            .await
            .map_err(RestError::RequestError)?;

        let status = response.status();
        let raw_text = response.text().await.map_err(RestError::RequestError)?;

        if !status.is_success() {
            return Err(RestError::HttpError {
                status,
                body: raw_text,
            });
        }

        serde_json::from_str(&raw_text).map_err(|e| RestError::ParseError {
            error: e,
            raw_json: raw_text,
        })
    }
    pub async fn get_tag_by_slug(
        &self,
        args: GetTagBySlugArgs,
    ) -> Result<GetTagBySlugResponse, RestError> {
        let path = format!("{}/tags/slug/{}", self.base_url, args.slug);

        let response = self
            .http_client
            .get(path)
            .query(&args)
            .send()
            .await
            .map_err(RestError::RequestError)?;

        let status = response.status();
        let raw_text = response.text().await.map_err(RestError::RequestError)?;

        if !status.is_success() {
            return Err(RestError::HttpError {
                status,
                body: raw_text,
            });
        }

        serde_json::from_str(&raw_text).map_err(|e| RestError::ParseError {
            error: e,
            raw_json: raw_text,
        })
    }
    pub async fn get_related_tags_by_id(
        &self,
        args: GetRelatedTagByIdArgs,
    ) -> Result<GetRelatedTagByIdResponse, RestError> {
        let path = format!("{}/tags/{}/related-tags", self.base_url, args.id);

        let response = self
            .http_client
            .get(path)
            .query(&args)
            .send()
            .await
            .map_err(RestError::RequestError)?;

        let status = response.status();
        let raw_text = response.text().await.map_err(RestError::RequestError)?;

        if !status.is_success() {
            return Err(RestError::HttpError {
                status,
                body: raw_text,
            });
        }

        serde_json::from_str(&raw_text).map_err(|e| RestError::ParseError {
            error: e,
            raw_json: raw_text,
        })
    }

    pub async fn get_related_tags_by_slug(
        &self,
        args: GetRelatedTagBySlugArgs,
    ) -> Result<GetRelatedTagBySlugResponse, RestError> {
        let path = format!("{}/tags/slug/{}/related-tags", self.base_url, args.slug);

        let response = self
            .http_client
            .get(path)
            .query(&args)
            .send()
            .await
            .map_err(RestError::RequestError)?;

        let status = response.status();
        let raw_text = response.text().await.map_err(RestError::RequestError)?;

        if !status.is_success() {
            return Err(RestError::HttpError {
                status,
                body: raw_text,
            });
        }

        serde_json::from_str(&raw_text).map_err(|e| RestError::ParseError {
            error: e,
            raw_json: raw_text,
        })
    }

    pub async fn get_tags_related_to_id(
        &self,
        args: GetTagsRelatedToIdArgs,
    ) -> Result<GetTagsRelatedToIdResponse, RestError> {
        let path = format!("{}/tags/{}/related-tags/tags", self.base_url, args.id);

        let response = self
            .http_client
            .get(path)
            .query(&args)
            .send()
            .await
            .map_err(RestError::RequestError)?;

        let status = response.status();
        let raw_text = response.text().await.map_err(RestError::RequestError)?;

        if !status.is_success() {
            return Err(RestError::HttpError {
                status,
                body: raw_text,
            });
        }

        serde_json::from_str(&raw_text).map_err(|e| RestError::ParseError {
            error: e,
            raw_json: raw_text,
        })
    }

    pub async fn get_tags_related_to_slug(
        &self,
        args: GetTagsRelatedToSlugArgs,
    ) -> Result<GetTagsRelatedToSlugResponse, RestError> {
        let path = format!(
            "{}/tags/slug/{}/related-tags/tags",
            self.base_url, args.slug
        );

        let response = self
            .http_client
            .get(path)
            .query(&args)
            .send()
            .await
            .map_err(RestError::RequestError)?;

        let status = response.status();
        let raw_text = response.text().await.map_err(RestError::RequestError)?;

        if !status.is_success() {
            return Err(RestError::HttpError {
                status,
                body: raw_text,
            });
        }

        serde_json::from_str(&raw_text).map_err(|e| RestError::ParseError {
            error: e,
            raw_json: raw_text,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        GammaClient,
        types::types_tags::{
            GetRelatedTagByIdArgs, GetRelatedTagBySlugArgs, GetTagByIdArgs, GetTagBySlugArgs,
            ListTagsArgs,
        },
    };

    #[tokio::test]
    async fn list_tags_test() {
        let client = GammaClient::new();

        let args = ListTagsArgs::default();
        let response = client.list_tags(args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn get_tag_by_id_test() {
        let client = GammaClient::new();

        let args = GetTagByIdArgs {
            id: 1,
            include_template: false,
        };
        let response = client.get_tag_by_id(args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn get_tag_by_slug_test() {
        let client = GammaClient::new();

        let args = GetTagBySlugArgs {
            slug: "mississippi".to_string(),
            include_template: false,
        };
        let response = client.get_tag_by_slug(args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn get_related_tags_by_id_test() {
        let client = GammaClient::new();

        let args = GetRelatedTagByIdArgs {
            id: 1,
            omit_empty: None,
            status: None,
        };

        let response = client.get_related_tags_by_id(args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn get_related_tags_by_slug_test() {
        let client = GammaClient::new();

        let args = GetRelatedTagBySlugArgs {
            slug: "sports".to_string(),
            omit_empty: None,
            status: None,
        };

        let response = client.get_related_tags_by_slug(args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }
}
