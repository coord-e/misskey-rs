#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod endpoint {
    pub use misskey_api::endpoint::*;
    pub use misskey_core::{Request, UploadFileRequest};
}

pub mod streaming {
    pub use misskey_api::streaming::*;
    pub use misskey_core::streaming::*;
}

pub mod model {
    pub use misskey_api::model::*;
    pub use misskey_core::model::*;
}

pub use misskey_core::Client;

#[cfg(feature = "http-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "http-client")))]
pub mod http {
    pub use misskey_http::*;
}

#[cfg(feature = "http-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "http-client")))]
pub use http::{HttpClient, HttpClientBuilder};

#[cfg(feature = "websocket-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "websocket-client")))]
pub mod websocket {
    pub use misskey_websocket::*;
}

#[cfg(feature = "websocket-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "websocket-client")))]
pub use websocket::{WebSocketClient, WebSocketClientBuilder};
