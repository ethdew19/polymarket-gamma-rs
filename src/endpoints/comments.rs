use crate::{
    GammaClient, RestError,
    types::types_comments::{
        GetCommentByCommentIdArgs, GetCommentsByCommentIdResponse, GetCommentsByUserAddressArgs,
        GetCommentsByUserAddressResponse, ListCommentsArgs, ListCommentsResponse,
    },
};

impl GammaClient {
    pub async fn list_comments(
        &self,
        args: ListCommentsArgs,
    ) -> Result<ListCommentsResponse, RestError> {
        let path = format!("{}/comments", self.base_url);
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
    pub async fn get_comments_by_comment_id(
        &self,
        args: GetCommentByCommentIdArgs,
    ) -> Result<GetCommentsByCommentIdResponse, RestError> {
        let path = format!("{}/comments/{}", self.base_url, args.id);
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
    pub async fn get_comments_by_user_address(
        &self,
        args: GetCommentsByUserAddressArgs,
    ) -> Result<GetCommentsByUserAddressResponse, RestError> {
        let path = format!(
            "{}/comments/user_address/{}",
            self.base_url, args.user_address
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
        types::types_comments::{
            GetCommentByCommentIdArgs, GetCommentsByUserAddressArgs, ListCommentsArgs,
        },
    };

    #[tokio::test]
    pub async fn list_comments_test() {
        let client = GammaClient::new();

        let args = ListCommentsArgs {
            parent_entity_type: "Series".to_string(),
            parent_entity_id: 1,
            ..Default::default()
        };

        let response = client.list_comments(args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    pub async fn get_comments_by_comment_id_test() {
        let client = GammaClient::new();

        let comment_id = 1015123;

        let args = GetCommentByCommentIdArgs {
            id: comment_id,
            get_positions: None,
        };

        let response = client.get_comments_by_comment_id(args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    pub async fn get_comments_by_user_address_test() {
        let client = GammaClient::new();

        let args = GetCommentsByUserAddressArgs {
            user_address: "0xaccdd45f1923800d2044aac517e6e18f249b80a3".to_string(),
            limit: None,
            offset: None,
            order: None,
            ascending: None,
        };

        let response = client.get_comments_by_user_address(args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }
}
