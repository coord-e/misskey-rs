#[cfg(feature = "12-27-0")]
use crate::Error;

#[cfg(feature = "12-27-0")]
use misskey_api::endpoint;
#[cfg(feature = "12-27-0")]
use misskey_core::Client;
#[cfg(feature = "12-27-0")]
use url::Url;

/// Builder for the [`build_notification`][`crate::ClientExt::build_notification`] method.
#[cfg(feature = "12-27-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-27-0")))]
pub struct NotificationBuilder<C> {
    client: C,
    request: endpoint::notifications::create::Request,
}

#[cfg(feature = "12-27-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-27-0")))]
impl<C> NotificationBuilder<C> {
    /// Creates a builder with the client.
    pub fn new(client: C) -> Self {
        let request = endpoint::notifications::create::Request {
            body: String::new(),
            header: None,
            icon: None,
        };
        NotificationBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::notifications::create::Request {
        &self.request
    }

    /// Sets the body text of the notification.
    pub fn body(&mut self, body: impl Into<String>) -> &mut Self {
        self.request.body = body.into();
        self
    }

    /// Sets the header text of the notification.
    pub fn header(&mut self, header: impl Into<String>) -> &mut Self {
        self.request.header.replace(header.into());
        self
    }

    /// Sets the icon of the notification.
    pub fn icon(&mut self, icon: Url) -> &mut Self {
        self.request.icon.replace(icon);
        self
    }
}

#[cfg(feature = "12-27-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-27-0")))]
impl<C: Client> NotificationBuilder<C> {
    /// Creates the notification.
    pub async fn create(&self) -> Result<(), Error<C::Error>> {
        self.client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(())
    }
}
