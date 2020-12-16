use crate::Error;

use misskey_api::model::{
    drive::DriveFile, messaging::MessagingMessage, user::User, user_group::UserGroup,
};
use misskey_api::{endpoint, EntityRef};
use misskey_core::Client;

/// Builder for the [`build_message`][`crate::ClientExt::build_message`] method.
pub struct MessagingMessageBuilder<C> {
    client: C,
    request: endpoint::messaging::messages::create::Request,
}

impl<C> MessagingMessageBuilder<C> {
    /// Creates a builder with the client.
    pub fn new(client: C) -> Self {
        let request = endpoint::messaging::messages::create::Request {
            text: None,
            user_id: None,
            group_id: None,
            file_id: None,
        };
        MessagingMessageBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::messaging::messages::create::Request {
        &self.request
    }

    /// Sets the text content of the message.
    pub fn text(&mut self, text: impl Into<String>) -> &mut Self {
        self.request.text.replace(text.into());
        self
    }

    /// Specifies the recipient user.
    ///
    /// Note that user and group cannot be specified as the recipient at the same time.
    /// Therefore, even if [`group`][`MessagingMessageBuilder::group`] is used before this method call,
    /// it will be overwritten and the message is only sent to the user specified in this call.
    pub fn user(&mut self, user: impl EntityRef<User>) -> &mut Self {
        self.request.user_id.replace(user.entity_ref());
        self.request.group_id.take();
        self
    }

    /// Specifies the recipient user group.
    ///
    /// Note that user and group cannot be specified as the recipient at the same time.
    /// Therefore, even if [`user`][`MessagingMessageBuilder::user`] is used before this method call,
    /// it will be overwritten and the message is only sent to the user group specified in this call.
    pub fn group(&mut self, group: impl EntityRef<UserGroup>) -> &mut Self {
        self.request.group_id.replace(group.entity_ref());
        self.request.user_id.take();
        self
    }

    /// Sets the file content of the message.
    pub fn file(&mut self, file: impl EntityRef<DriveFile>) -> &mut Self {
        self.request.file_id.replace(file.entity_ref());
        self
    }
}

impl<C: Client> MessagingMessageBuilder<C> {
    /// Creates the message.
    pub async fn create(&self) -> Result<MessagingMessage, Error<C::Error>> {
        let message = self
            .client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(message)
    }
}
