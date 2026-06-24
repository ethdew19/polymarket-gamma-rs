use crate::{
    client::GammaClient,
    error::Result,
    types::types_events::{
        GetEventByIdArgs, GetEventByIdResponse, GetEventBySlugArgs, GetEventBySlugResponse,
        GetEventTagsResponse, ListEventsArgs, ListEventsKeysetArgs, ListEventsKeysetResponse,
        ListEventsResponse,
    },
};

impl GammaClient {
    pub async fn list_events(&self, args: &ListEventsArgs) -> Result<ListEventsResponse> {
        self.get("events", args).await
    }

    pub async fn list_events_keyset(
        &self,
        args: &ListEventsKeysetArgs,
    ) -> Result<ListEventsKeysetResponse> {
        self.get("events/keyset", args).await
    }

    pub async fn get_event_by_id(&self, args: &GetEventByIdArgs) -> Result<GetEventByIdResponse> {
        self.get(&format!("events/{}", args.id), args).await
    }

    pub async fn get_event_tags(&self, id: u64) -> Result<GetEventTagsResponse> {
        self.get(&format!("events/{}/tags", id), &()).await
    }

    pub async fn get_event_by_slug(
        &self,
        args: &GetEventBySlugArgs,
    ) -> Result<GetEventBySlugResponse> {
        self.get(&format!("events/slug/{}", args.slug), args).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        client::GammaClient,
        types::types_events::{
            GetEventByIdArgs, GetEventBySlugArgs, ListEventsArgs, ListEventsKeysetArgs,
        },
    };

    #[tokio::test]
    async fn list_events_test() {
        let client = GammaClient::default();

        let args = ListEventsArgs::default();
        let response = client.list_events(&args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn get_event_by_id_test() {
        let client = GammaClient::default();

        let args = GetEventByIdArgs {
            id: 2909,
            include_chat: None,
            include_template: None,
        };
        let response = client.get_event_by_id(&args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn get_event_tags_test() {
        let client = GammaClient::default();

        let id = 2909;

        let response = client.get_event_tags(id).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn list_events_keyset_pagination_test() {
        let client = GammaClient::default();
        let pages = 3;
        let per_page = 5;

        let mut cursor = None;
        let mut total = 0;

        for page in 0..pages {
            let args = ListEventsKeysetArgs {
                limit: Some(per_page),
                after_cursor: cursor.clone(),
                ..Default::default()
            };
            let response = client.list_events_keyset(&args).await.unwrap();

            assert!(!response.events.is_empty(), "page {} returned no events", page);
            assert!(
                response.events.len() <= per_page as usize,
                "page {} returned more than limit",
                page,
            );
            total += response.events.len();

            println!(
                "page {}: {} events, next_cursor: {:?}",
                page,
                response.events.len(),
                response.next_cursor,
            );

            match response.next_cursor {
                Some(c) => cursor = Some(c),
                None => break,
            }
        }

        assert!(total > per_page as usize, "should have fetched more than one page of events");
    }

    #[tokio::test]
    async fn get_event_by_slug_test() {
        let client = GammaClient::default();

        let args = GetEventBySlugArgs {
            slug:
                "will-surojit-chatterjee-or-matt-huang-win-in-their-cryptochamps-finals-chess-match"
                    .to_string(),
            include_chat: false,
            include_template: false,
        };

        let response = client.get_event_by_slug(&args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }
}
