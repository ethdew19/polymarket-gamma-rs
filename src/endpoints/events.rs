use crate::{
    GammaClient, RestError,
    types::types_events::{
        GetEventByIdArgs, GetEventByIdResponse, GetEventBySlugArgs, GetEventBySlugResponse,
        GetEventTagsResponse, ListEventsArgs, ListEventsResponse,
    },
};

impl GammaClient {
    pub async fn list_events(&self, args: ListEventsArgs) -> Result<ListEventsResponse, RestError> {
        let path = format!("{}/events", self.base_url);
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
    pub async fn get_event_by_id(
        &self,
        args: GetEventByIdArgs,
    ) -> Result<GetEventByIdResponse, RestError> {
        let path = format!("{}/events/{}", self.base_url, args.id);

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
    pub async fn get_event_tags(&self, id: u64) -> Result<GetEventTagsResponse, RestError> {
        let path = format!("{}/events/{}/tags", self.base_url, id);

        let response = self
            .http_client
            .get(path)
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
    pub async fn get_event_by_slug(
        &self,
        args: GetEventBySlugArgs,
    ) -> Result<GetEventBySlugResponse, RestError> {
        let path = format!("{}/events/slug/{}", self.base_url, args.slug);

        let response = self
            .http_client
            .get(path)
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
        types::types_events::{GetEventByIdArgs, GetEventBySlugArgs, ListEventsArgs},
    };

    #[tokio::test]
    async fn list_events_test() {
        let client = GammaClient::new();

        let args = ListEventsArgs::default();
        let response = client.list_events(args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn get_event_by_id_test() {
        let client = GammaClient::new();

        let args = GetEventByIdArgs {
            id: 2909,
            include_chat: None,
            include_template: None,
        };
        let response = client.get_event_by_id(args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn get_event_tags_test() {
        let client = GammaClient::new();

        let id = 2909;

        let response = client.get_event_tags(id).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn get_event_by_slug_test() {
        let client = GammaClient::new();

        let args = GetEventBySlugArgs {
            slug:
                "will-surojit-chatterjee-or-matt-huang-win-in-their-cryptochamps-finals-chess-match"
                    .to_string(),
            include_chat: false,
            include_template: false,
        };

        let response = client.get_event_by_slug(args).await;

        println!("{:?}", response);

        assert!(response.is_ok());
    }
}
