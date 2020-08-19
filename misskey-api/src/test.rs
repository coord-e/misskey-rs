use misskey_core::{ApiRequest, Client};
use misskey_http::HttpClient;
use url::Url;

pub struct TestClient(HttpClient);

impl TestClient {
    pub fn new() -> Self {
        let url = std::env::var("TEST_API_URL").unwrap();
        let token = std::env::var("TEST_API_TOKEN").unwrap();
        TestClient(HttpClient::new(Url::parse(&url).unwrap(), Some(token)))
    }

    pub async fn test<R: ApiRequest + Send>(&mut self, req: R) -> R::Response {
        self.0.request(req).await.unwrap().unwrap()
    }
}
