use crate::model::{emoji::Emoji, id::Id};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub ids: Vec<Id<Emoji>>,
    pub aliases: Vec<String>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "admin/emoji/remove-aliases-bulk";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        use crate::model::{emoji::Emoji, id::Id};

        let client = TestClient::new();
        let ids: Vec<Id<Emoji>> = client
            .admin
            .test(crate::endpoint::admin::emoji::list::Request::default())
            .await
            .iter()
            .map(|emoji| emoji.id)
            .collect();

        client
            .admin
            .test(crate::endpoint::admin::emoji::add_aliases_bulk::Request {
                ids: ids.clone(),
                aliases: vec!["alias1".to_string(), "alias2".to_string()],
            })
            .await;

        client
            .admin
            .test(Request {
                ids,
                aliases: vec!["alias1".to_string()],
            })
            .await;
    }
}
