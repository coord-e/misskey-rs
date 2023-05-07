use crate::Error;

use chrono::Utc;
use misskey_api::model::drive::DriveFile;
use misskey_api::model::page::{Content, Font, Page, Variables};
use misskey_api::{endpoint, EntityRef};
use misskey_core::Client;

/// Builder for the [`build_page`][`crate::ClientExt::build_page`] method.
pub struct PageBuilder<C> {
    client: C,
    request: endpoint::pages::create::Request,
}

impl<C> PageBuilder<C> {
    /// Creates a builder with the client.
    pub fn new(client: C) -> Self {
        let request = endpoint::pages::create::Request {
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
        };
        PageBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::pages::create::Request {
        &self.request
    }

    /// Sets the title of the page.
    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        self.request.title = title.into();
        self
    }

    /// Sets the name of the page.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.request.name = name.into();
        self
    }

    /// Sets the summary of the page.
    pub fn summary(&mut self, summary: impl Into<String>) -> &mut Self {
        self.request.summary.replace(summary.into());
        self
    }

    /// Sets the content of the page.
    pub fn content(&mut self, content: Content) -> &mut Self {
        self.request.content = content;
        self
    }

    /// Sets the variables of the page.
    pub fn variables(&mut self, variables: Variables) -> &mut Self {
        self.request.variables = variables;
        self
    }

    #[cfg(feature = "12-31-0")]
    /// Sets the script of the page.
    pub fn script(&mut self, script: impl Into<String>) -> &mut Self {
        self.request.script = script.into();
        self
    }

    /// Sets the eye catching image of the page.
    pub fn eye_catching_image(&mut self, file: impl EntityRef<DriveFile>) -> &mut Self {
        let file_id = file.entity_ref();
        self.request.eye_catching_image_id.replace(file_id);
        self
    }

    /// Sets the font of the page.
    pub fn font(&mut self, font: Font) -> &mut Self {
        self.request.font.replace(font);
        self
    }

    /// Sets the font of the page to serif.
    pub fn serif(&mut self) -> &mut Self {
        self.font(Font::Serif)
    }

    /// Sets the font of the page to sans-serif.
    pub fn sans_serif(&mut self) -> &mut Self {
        self.font(Font::SansSerif)
    }

    /// Sets whether or not to center page elements.
    pub fn align_center(&mut self, align_center: bool) -> &mut Self {
        self.request.align_center.replace(align_center);
        self
    }

    /// Sets whether or not to hide page title when pinned to profile.
    pub fn hide_title_when_pinned(&mut self, hide_title_when_pinned: bool) -> &mut Self {
        self.request
            .hide_title_when_pinned
            .replace(hide_title_when_pinned);
        self
    }
}

impl<C: Client> PageBuilder<C> {
    /// Creates the page.
    pub async fn create(&self) -> Result<Page, Error<C::Error>> {
        let page = self
            .client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(page)
    }
}

/// Builder for the [`update_page`][`crate::ClientExt::update_page`] method.
pub struct PageUpdateBuilder<C> {
    client: C,
    request: endpoint::pages::update::Request,
}

impl<C> PageUpdateBuilder<C> {
    /// Creates a builder with the client and the page you are going to update.
    pub fn new(client: C, page: Page) -> Self {
        let Page {
            id,
            content,
            variables,
            title,
            name,
            summary,
            align_center,
            hide_title_when_pinned,
            font,
            #[cfg(feature = "12-31-0")]
            script,
            eye_catching_image_id,
            ..
        } = page;
        let request = endpoint::pages::update::Request {
            page_id: id,
            title,
            name,
            summary,
            content,
            variables,
            #[cfg(feature = "12-31-0")]
            script,
            eye_catching_image_id,
            font: Some(font),
            align_center: Some(align_center),
            hide_title_when_pinned: Some(hide_title_when_pinned),
        };
        PageUpdateBuilder { client, request }
    }

    /// Gets the request object for reuse.
    pub fn as_request(&self) -> &endpoint::pages::update::Request {
        &self.request
    }

    /// Sets the title of the page.
    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        self.request.title = title.into();
        self
    }

    /// Sets the name of the page.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.request.name = name.into();
        self
    }

    /// Sets the summary of the page.
    pub fn summary(&mut self, summary: impl Into<String>) -> &mut Self {
        self.request.summary.replace(summary.into());
        self
    }

    /// Sets the content of the page.
    pub fn content(&mut self, content: Content) -> &mut Self {
        self.request.content = content;
        self
    }

    /// Sets the variables of the page.
    pub fn variables(&mut self, variables: Variables) -> &mut Self {
        self.request.variables = variables;
        self
    }

    #[cfg(feature = "12-31-0")]
    /// Sets the script of the page.
    pub fn script(&mut self, script: impl Into<String>) -> &mut Self {
        self.request.script = script.into();
        self
    }

    /// Sets the eye catching image of the page.
    pub fn eye_catching_image(&mut self, file: impl EntityRef<DriveFile>) -> &mut Self {
        let file_id = file.entity_ref();
        self.request.eye_catching_image_id.replace(file_id);
        self
    }

    /// Sets the font of the page.
    pub fn font(&mut self, font: Font) -> &mut Self {
        self.request.font.replace(font);
        self
    }

    /// Sets the font of the page to serif.
    pub fn serif(&mut self) -> &mut Self {
        self.font(Font::Serif)
    }

    /// Sets the font of the page to sans-serif.
    pub fn sans_serif(&mut self) -> &mut Self {
        self.font(Font::SansSerif)
    }

    /// Sets whether or not to center page elements.
    pub fn align_center(&mut self, align_center: bool) -> &mut Self {
        self.request.align_center.replace(align_center);
        self
    }

    /// Sets whether or not to hide page title when pinned to profile.
    pub fn hide_title_when_pinned(&mut self, hide_title_when_pinned: bool) -> &mut Self {
        self.request
            .hide_title_when_pinned
            .replace(hide_title_when_pinned);
        self
    }
}

impl<C: Client> PageUpdateBuilder<C> {
    /// Updates the page.
    pub async fn update(&self) -> Result<(), Error<C::Error>> {
        self.client
            .request(&self.request)
            .await
            .map_err(Error::Client)?
            .into_result()?;
        Ok(())
    }
}
