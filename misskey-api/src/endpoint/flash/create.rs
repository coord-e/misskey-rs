use crate::model::flash::Flash;

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[builder(default, setter(into))]
    pub title: String,
    #[builder(default, setter(into))]
    pub summary: String,
    #[builder(default, setter(into))]
    pub script: String,
    #[builder(default, setter(into))]
    pub permissions: Vec<String>,
}

impl misskey_core::Request for Request {
    type Response = Flash;
    const ENDPOINT: &'static str = "flash/create";
}

#[cfg(test)]
mod tests {
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
        client
            .test(Request {
                title: "play".to_string(),
                summary: "summary".to_string(),
                script: r#"/// @ 0.12.2

                var name = ""

                Ui:render([
                    Ui:C:textInput({
                        label: "Your name"
                        onInput: @(v) { name = v }
                    })
                    Ui:C:button({
                        text: "Hello"
                        onClick: @() {
                            Mk:dialog(null `Hello, {name}!`)
                        }
                    })
                ])
                "#
                .to_string(),
                permissions: vec!["read:account".to_string()],
            })
            .await;
    }
}
