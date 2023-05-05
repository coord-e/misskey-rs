use crate::Error;

use chrono::{DateTime, Duration, Utc};
#[cfg(feature = "12-47-0")]
use misskey_api::model::channel::Channel;
#[cfg(feature = "13-10-0")]
use misskey_api::model::note::ReactionAcceptance;
use misskey_api::model::{
    drive::DriveFile,
    note::{Note, Visibility},
    user::User,
};
use misskey_api::{endpoint, EntityRef};
use misskey_core::Client;

fn initial_notes_create_request() -> endpoint::notes::create::Request {
    endpoint::notes::create::Request {
        visibility: Some(Visibility::Public),
        visible_user_ids: None,
        text: None,
        cw: None,
        #[cfg(not(feature = "12-96-0"))]
        via_mobile: Some(false),
        local_only: Some(false),
        #[cfg(feature = "13-10-0")]
        #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
        reaction_acceptance: None,
        no_extract_mentions: Some(false),
        no_extract_hashtags: Some(false),
        no_extract_emojis: Some(false),
        file_ids: None,
        reply_id: None,
        renote_id: None,
        poll: None,
        #[cfg(feature = "12-47-0")]
        channel_id: None,
    }
}

fn initial_poll_request() -> endpoint::notes::create::PollRequest {
    endpoint::notes::create::PollRequest {
        choices: Vec::new(),
        multiple: Some(false),
        expires_at: None,
        expired_after: None,
    }
}

/// Builder for the [`build_note`][`crate::ClientExt::build_note`] method.
pub struct NoteBuilder<C> {
    client: C,
    request: endpoint::notes::create::Request,
}

impl<C> NoteBuilder<C> {
    /// Creates a builder with the client.
    pub fn new(client: C) -> Self {
        NoteBuilder {
            client,
            request: initial_notes_create_request(),
        }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::notes::create::Request {
        &self.request
    }

    /// Sets the text of the note.
    pub fn text(&mut self, text: impl Into<String>) -> &mut Self {
        self.request.text.replace(text.into());
        self
    }

    /// Creates a poll and sets the choices.
    pub fn poll(&mut self, choices: impl IntoIterator<Item = impl Into<String>>) -> &mut Self {
        let choices = choices.into_iter().map(Into::into).collect();
        if let Some(poll) = self.request.poll.as_mut() {
            poll.choices = choices;
        } else {
            self.request.poll = Some(endpoint::notes::create::PollRequest {
                choices,
                ..initial_poll_request()
            });
        }
        self
    }

    /// Sets whether to allow multiple votes.
    pub fn poll_multiple(&mut self, multiple: bool) -> &mut Self {
        if let Some(poll) = self.request.poll.as_mut() {
            poll.multiple.replace(multiple);
        } else {
            self.request.poll = Some(endpoint::notes::create::PollRequest {
                multiple: Some(multiple),
                ..initial_poll_request()
            });
        }
        self
    }

    /// Sets when the vote will expire.
    pub fn poll_expires_at(&mut self, at: DateTime<Utc>) -> &mut Self {
        if let Some(poll) = self.request.poll.as_mut() {
            poll.expires_at.replace(at);
            poll.expired_after.take();
        } else {
            self.request.poll = Some(endpoint::notes::create::PollRequest {
                expires_at: Some(at),
                expired_after: None,
                ..initial_poll_request()
            });
        }
        self
    }

    /// Sets how long the vote will expire after.
    pub fn poll_expires_after(&mut self, after: Duration) -> &mut Self {
        if let Some(poll) = self.request.poll.as_mut() {
            poll.expired_after.replace(after);
            poll.expires_at.take();
        } else {
            self.request.poll = Some(endpoint::notes::create::PollRequest {
                expired_after: Some(after),
                expires_at: None,
                ..initial_poll_request()
            });
        }
        self
    }

    /// Attaches the specified file to the note.
    pub fn attach_file(&mut self, file: impl EntityRef<DriveFile>) -> &mut Self {
        let file_id = file.entity_ref();
        if let Some(ids) = self.request.file_ids.as_mut() {
            ids.push(file_id);
        } else {
            self.request.file_ids = Some(vec![file_id]);
        }
        self
    }

    /// Adds attachments to the note.
    pub fn attach_files(
        &mut self,
        files: impl IntoIterator<Item = impl EntityRef<DriveFile>>,
    ) -> &mut Self {
        let it = files.into_iter().map(|file| file.entity_ref());
        if let Some(ids) = self.request.file_ids.as_mut() {
            ids.extend(it);
        } else {
            self.request.file_ids = Some(it.collect());
        }
        self
    }

    /// Sets the visibility of the note.
    pub fn visibility(&mut self, visibility: Visibility) -> &mut Self {
        self.request.visibility = Some(visibility);
        if !matches!(visibility, Visibility::Specified) {
            self.request.visible_user_ids.take();
        }
        self
    }

    /// Sets the note to be visible to everyone.
    ///
    /// This is equivalent to `.visibility(Visibility::Public)`.
    pub fn public(&mut self) -> &mut Self {
        self.visibility(Visibility::Public)
    }

    /// Sets the note to be visible only to the home timeline.
    ///
    /// This is equivalent to `.visibility(Visibility::Home)`.
    pub fn home_only(&mut self) -> &mut Self {
        self.visibility(Visibility::Home)
    }

    /// Sets the note to be visible only to the followers.
    ///
    /// This is equivalent to `.visibility(Visibility::Followers)`.
    pub fn followers_only(&mut self) -> &mut Self {
        self.visibility(Visibility::Followers)
    }

    /// Sets the note to be visible only to the specified users.
    pub fn direct(
        &mut self,
        recipients: impl IntoIterator<Item = impl EntityRef<User>>,
    ) -> &mut Self {
        self.visibility(Visibility::Specified);
        self.request.visible_user_ids.replace(
            recipients
                .into_iter()
                .map(|user| user.entity_ref())
                .collect(),
        );
        self
    }

    /// Hides the contents of the note as a content warning with the specified text.
    pub fn hide_content(&mut self, cw_text: impl Into<String>) -> &mut Self {
        self.request.cw.replace(cw_text.into());
        self
    }

    #[cfg(not(feature = "12-96-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-96-0"))))]
    /// Sets whether to show the note as posted from a mobile device.
    pub fn via_mobile(&mut self, via_mobile: bool) -> &mut Self {
        self.request.via_mobile.replace(via_mobile);
        self
    }

    /// Sets the note to be visible only to users on the same instance.
    pub fn local_only(&mut self, local_only: bool) -> &mut Self {
        self.request.local_only.replace(local_only);
        self
    }

    /// Sets the reaction acceptance of the note.
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    pub fn reaction_acceptance(&mut self, reaction_acceptance: ReactionAcceptance) -> &mut Self {
        self.request
            .reaction_acceptance
            .replace(reaction_acceptance);
        self
    }

    /// Sets the note to reject any reactions other than likes.
    ///
    /// This is equivalent to `.reaction_acceptance(ReactionAcceptance::LikesOnly)`.
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    pub fn accept_only_likes(&mut self) -> &mut Self {
        self.reaction_acceptance(ReactionAcceptance::LikeOnly)
    }

    /// Sets the note to reject any reactions from remote servers that are not likes.
    ///
    /// This is equivalent to `.reaction_acceptance(ReactionAcceptance::LikesOnlyForRemote)`.
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    pub fn accept_only_likes_for_remote(&mut self) -> &mut Self {
        self.reaction_acceptance(ReactionAcceptance::LikeOnlyForRemote)
    }

    /// Sets the note to receive all reactions.
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    pub fn accept_all_reactions(&mut self) -> &mut Self {
        self.request.reaction_acceptance = None;
        self
    }

    /// Sets whether or not to extract mentions (i.e. `@username`) from the text of the note.
    ///
    /// Mentions are extracted by default, and you would need this method if you want to disable this behavior.
    pub fn extract_mentions(&mut self, extract_mentions: bool) -> &mut Self {
        self.request.no_extract_mentions.replace(!extract_mentions);
        self
    }

    /// Sets whether or not to extract hashtags (i.e. `#tag`) from the text of the note.
    ///
    /// Hashtags are extracted by default, and you would need this method if you want to disable this behavior.
    pub fn extract_hashtags(&mut self, extract_hashtags: bool) -> &mut Self {
        self.request.no_extract_hashtags.replace(!extract_hashtags);
        self
    }

    /// Sets whether or not to extract emojis (i.e. `:emoji:`) from the text of the note.
    ///
    /// Emojis are extracted by default, and you would need this method if you want to disable this behavior.
    pub fn extract_emojis(&mut self, extract_emojis: bool) -> &mut Self {
        self.request.no_extract_emojis.replace(!extract_emojis);
        self
    }

    /// Sets the note as a reply to the specified note.
    pub fn reply(&mut self, note: impl EntityRef<Note>) -> &mut Self {
        self.request.reply_id.replace(note.entity_ref());
        self
    }

    /// Sets the note as a renote of the specified note.
    pub fn renote(&mut self, note: impl EntityRef<Note>) -> &mut Self {
        self.request.renote_id.replace(note.entity_ref());
        self
    }

    /// Sets the note to be posted to the specified channel.
    #[cfg(feature = "12-47-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
    pub fn channel(&mut self, channel: impl EntityRef<Channel>) -> &mut Self {
        self.request.channel_id.replace(channel.entity_ref());
        self
    }
}

impl<C: Client> NoteBuilder<C> {
    /// Creates the note.
    pub async fn create(&self) -> Result<Note, Error<C::Error>> {
        let response = self
            .client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(response.created_note)
    }
}
