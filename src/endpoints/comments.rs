use crate::{
    client::GammaClient,
    error::Result,
    types::types_comments::{
        GetCommentByCommentIdArgs, GetCommentsByCommentIdResponse, GetCommentsByUserAddressArgs,
        GetCommentsByUserAddressResponse, ListCommentsArgs, ListCommentsResponse,
    },
};

impl GammaClient {
    pub async fn list_comments(&self, args: &ListCommentsArgs) -> Result<ListCommentsResponse> {
        self.get("comments", args).await
    }

    pub async fn get_comments_by_comment_id(
        &self,
        args: &GetCommentByCommentIdArgs,
    ) -> Result<GetCommentsByCommentIdResponse> {
        self.get(&format!("comments/{}", args.id), args).await
    }

    pub async fn get_comments_by_user_address(
        &self,
        args: &GetCommentsByUserAddressArgs,
    ) -> Result<GetCommentsByUserAddressResponse> {
        self.get(
            &format!("comments/user_address/{}", args.user_address),
            args,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        client::GammaClient,
        types::types_comments::{
            GetCommentByCommentIdArgs, GetCommentsByUserAddressArgs, ListCommentsArgs,
        },
    };

    #[tokio::test]
    pub async fn list_comments_test() {
        let client = GammaClient::default();

        let args = ListCommentsArgs {
            parent_entity_type: "Series".to_string(),
            parent_entity_id: 1,
            ..Default::default()
        };

        let response = client.list_comments(&args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    pub async fn get_comments_by_comment_id_test() {
        let client = GammaClient::default();

        let comment_id = 1015123;

        let args = GetCommentByCommentIdArgs {
            id: comment_id,
            get_positions: None,
        };

        let response = client.get_comments_by_comment_id(&args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    pub async fn get_comments_by_user_address_test() {
        let client = GammaClient::default();

        let args = GetCommentsByUserAddressArgs {
            user_address: "0xaccdd45f1923800d2044aac517e6e18f249b80a3".to_string(),
            limit: None,
            offset: None,
            order: None,
            ascending: None,
        };

        let response = client.get_comments_by_user_address(&args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }
}
