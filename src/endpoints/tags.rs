use crate::{
    client::GammaClient,
    error::Result,
    types::types_tags::{
        GetRelatedTagByIdArgs, GetRelatedTagByIdResponse, GetRelatedTagBySlugArgs,
        GetRelatedTagBySlugResponse, GetTagByIdArgs, GetTagByIdResponse, GetTagBySlugArgs,
        GetTagBySlugResponse, GetTagsRelatedToIdArgs, GetTagsRelatedToIdResponse,
        GetTagsRelatedToSlugArgs, GetTagsRelatedToSlugResponse, ListTagsArgs, ListTagsResponse,
    },
};

impl GammaClient {
    pub async fn list_tags(&self, args: &ListTagsArgs) -> Result<ListTagsResponse> {
        self.get("tags", args).await
    }

    pub async fn get_tag_by_id(&self, args: &GetTagByIdArgs) -> Result<GetTagByIdResponse> {
        self.get(&format!("tags/{}", args.id), args).await
    }

    pub async fn get_tag_by_slug(&self, args: &GetTagBySlugArgs) -> Result<GetTagBySlugResponse> {
        self.get(&format!("tags/slug/{}", args.slug), args).await
    }

    pub async fn get_related_tags_by_id(
        &self,
        args: &GetRelatedTagByIdArgs,
    ) -> Result<GetRelatedTagByIdResponse> {
        self.get(&format!("tags/{}/related-tags", args.id), args)
            .await
    }

    pub async fn get_related_tags_by_slug(
        &self,
        args: &GetRelatedTagBySlugArgs,
    ) -> Result<GetRelatedTagBySlugResponse> {
        self.get(&format!("tags/slug/{}/related-tags", args.slug), args)
            .await
    }

    pub async fn get_tags_related_to_id(
        &self,
        args: &GetTagsRelatedToIdArgs,
    ) -> Result<GetTagsRelatedToIdResponse> {
        self.get(&format!("tags/{}/related-tags/tags", args.id), args)
            .await
    }

    pub async fn get_tags_related_to_slug(
        &self,
        args: &GetTagsRelatedToSlugArgs,
    ) -> Result<GetTagsRelatedToSlugResponse> {
        self.get(&format!("tags/slug/{}/related-tags/tags", args.slug), args)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        client::GammaClient,
        types::types_tags::{
            GetRelatedTagByIdArgs, GetRelatedTagBySlugArgs, GetTagByIdArgs, GetTagBySlugArgs,
            ListTagsArgs,
        },
    };

    #[tokio::test]
    async fn list_tags_test() {
        let client = GammaClient::default();

        let args = ListTagsArgs::default();
        let response = client.list_tags(&args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn get_tag_by_id_test() {
        let client = GammaClient::default();

        let args = GetTagByIdArgs {
            id: 1,
            include_template: false,
        };
        let response = client.get_tag_by_id(&args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn get_tag_by_slug_test() {
        let client = GammaClient::default();

        let args = GetTagBySlugArgs {
            slug: "mississippi".to_string(),
            include_template: false,
        };
        let response = client.get_tag_by_slug(&args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn get_related_tags_by_id_test() {
        let client = GammaClient::default();

        let args = GetRelatedTagByIdArgs {
            id: 1,
            omit_empty: None,
            status: None,
        };

        let response = client.get_related_tags_by_id(&args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn get_related_tags_by_slug_test() {
        let client = GammaClient::default();

        let args = GetRelatedTagBySlugArgs {
            slug: "sports".to_string(),
            omit_empty: None,
            status: None,
        };

        let response = client.get_related_tags_by_slug(&args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }
}
