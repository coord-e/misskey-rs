use crate::model::{
    drive::DriveFile,
    id::Id,
    page::{Content, Font, Page, Variables},
};

use chrono::Utc;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[builder(default, setter(into))]
    pub title: String,
    /// [ 1 .. ] characters
    #[builder(default, setter(into))]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub summary: Option<String>,
    #[builder(default, setter(into))]
    pub content: Content,
    #[builder(default, setter(into))]
    pub variables: Variables,
    #[cfg(feature = "12-31-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-31-0")))]
    #[builder(default, setter(into))]
    pub script: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub eye_catching_image_id: Option<Id<DriveFile>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub align_center: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub hide_title_when_pinned: Option<bool>,
}

impl Default for Request {
    fn default() -> Self {
        Request {
            title: String::default(),
            name: Utc::now().timestamp_millis().to_string(),
            summary: None,
            content: Content::default(),
            variables: Variables::default(),
            #[cfg(feature = "12-31-0")]
            script: String::default(),
            eye_catching_image_id: None,
            font: None,
            align_center: None,
            hide_title_when_pinned: None,
        }
    }
}

impl misskey_core::Request for Request {
    type Response = Page;
    const ENDPOINT: &'static str = "pages/create";
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_options() {
        let client = TestClient::new();
        let url = client.avatar_url().await;
        let file = client.upload_from_url(url).await;

        client
            .test(Request {
                title: "page".to_string(),
                name: Utc::now().timestamp_millis().to_string(),
                summary: Some("page summary".to_string()),
                content: r#"[
                    {
                        "type": "text",
                        "text": "Hello World!"
                    },
                    {
                        "type": "section",
                        "title": "Section 1",
                        "children": [
                            {
                                "type": "text",
                                "text": "text in section"
                            }
                        ]
                    }
                ]"#
                .parse()
                .unwrap(),
                variables: r#"[
                    {
                        "name": "x",
                        "type": "number",
                        "value": "1"
                    }
                ]"#
                .parse()
                .unwrap(),
                #[cfg(feature = "12-31-0")]
                script: r#"<: "Hello, world!""#.to_string(),
                eye_catching_image_id: Some(file.id),
                font: Some("serif".parse().unwrap()),
                align_center: Some(true),
                hide_title_when_pinned: Some(true),
            })
            .await;
    }
}
