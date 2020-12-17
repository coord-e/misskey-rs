/// Paginated item type in [`PaginationRequest`].
pub trait PaginationItem {
    /// The ID type.
    type Id: Ord;
    /// Extracts an ID from the item.
    fn item_id(&self) -> Self::Id;
}

/// [`Request`][`misskey_core::Request`] that can be paginated via `since_id` and `until_id`.
pub trait PaginationRequest: misskey_core::Request {
    /// The paginated item type.
    type Item: PaginationItem;

    /// Sets the `since_id` field of the request.
    fn set_since_id(&mut self, since_id: <Self::Item as PaginationItem>::Id);
    /// Sets the `until_id` field of the request.
    fn set_until_id(&mut self, until_id: <Self::Item as PaginationItem>::Id);
    /// Sets the `limit` field of the request.
    fn set_limit(&mut self, limit: u8);
}

impl<R: ?Sized> PaginationRequest for &'_ mut R
where
    R: PaginationRequest,
{
    type Item = R::Item;

    fn set_since_id(&mut self, since_id: <Self::Item as PaginationItem>::Id) {
        R::set_since_id(self, since_id)
    }
    fn set_until_id(&mut self, until_id: <Self::Item as PaginationItem>::Id) {
        R::set_until_id(self, until_id)
    }
    fn set_limit(&mut self, limit: u8) {
        R::set_limit(self, limit)
    }
}

impl<R: ?Sized> PaginationRequest for Box<R>
where
    R: PaginationRequest,
{
    type Item = R::Item;

    fn set_since_id(&mut self, since_id: <Self::Item as PaginationItem>::Id) {
        R::set_since_id(self, since_id)
    }
    fn set_until_id(&mut self, until_id: <Self::Item as PaginationItem>::Id) {
        R::set_until_id(self, until_id)
    }
    fn set_limit(&mut self, limit: u8) {
        R::set_limit(self, limit)
    }
}

/// [`Request`][`misskey_core::Request`] that can be paginated via `offset`.
pub trait OffsetPaginationRequest: misskey_core::Request {
    /// The paginated item type.
    type Item;

    /// Sets the `offset` field of the request.
    fn set_offset(&mut self, offset: u64);
    /// Sets the `limit` field of the request.
    fn set_limit(&mut self, limit: u8);
}

impl<R: ?Sized> OffsetPaginationRequest for &'_ mut R
where
    R: OffsetPaginationRequest,
{
    type Item = R::Item;

    fn set_offset(&mut self, offset: u64) {
        R::set_offset(self, offset)
    }
    fn set_limit(&mut self, limit: u8) {
        R::set_limit(self, limit)
    }
}

impl<R: ?Sized> OffsetPaginationRequest for Box<R>
where
    R: OffsetPaginationRequest,
{
    type Item = R::Item;

    fn set_offset(&mut self, offset: u64) {
        R::set_offset(self, offset)
    }
    fn set_limit(&mut self, limit: u8) {
        R::set_limit(self, limit)
    }
}
