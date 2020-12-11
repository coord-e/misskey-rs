//! Implementation of pagination.
//!
//! This module implements pagination using a request that implements
//! [`PaginationRequest`][pagination_request].
//! [`Pager`] trait serves as an alias for [`Stream`][stream] that fetches each page one by
//! one, and [`PagerStream`] wraps it into [`Stream`][stream] that takes it by element.
//!
//! [pagination_request]: misskey_api::PaginationRequest
//! [stream]: futures::stream::Stream
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::Error;

use futures::{
    future::{BoxFuture, FutureExt},
    stream::{FusedStream, Stream, StreamExt},
};
use misskey_api::{OffsetPaginationRequest, PaginationItem, PaginationRequest};
use misskey_core::model::ApiResult;
use misskey_core::{Client, Request};

enum PagerState<'a, C: Client + ?Sized, R: Request> {
    Pending {
        request: R,
        request_future: BoxFuture<'a, Result<ApiResult<R::Response>, C::Error>>,
    },
    Ready {
        next_request: R,
    },
}

pub(crate) struct BackwardPager<'a, C: Client + ?Sized, R: PaginationRequest> {
    client: &'a C,
    since_id: Option<<R::Item as PaginationItem>::Id>,
    state: Option<PagerState<'a, C, R>>,
}

impl<'a, C: Client + ?Sized, R: PaginationRequest> BackwardPager<'a, C, R> {
    pub(crate) fn with_since_id(
        client: &'a C,
        since_id: Option<<R::Item as PaginationItem>::Id>,
        request: R,
    ) -> Self {
        BackwardPager {
            client,
            since_id,
            state: Some(PagerState::Ready {
                next_request: request,
            }),
        }
    }

    pub(crate) fn new(client: &'a C, request: R) -> Self {
        BackwardPager {
            client,
            since_id: None,
            state: Some(PagerState::Ready {
                next_request: request,
            }),
        }
    }
}

impl<'a, C, R> Stream for BackwardPager<'a, C, R>
where
    C: Client + ?Sized,
    R: PaginationRequest + Unpin,
    R::Response: IntoIterator<Item = R::Item>,
    <R::Item as PaginationItem>::Id: Clone + Unpin,
{
    type Item = Result<Vec<R::Item>, Error<C::Error>>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let state = self.state.take();
        match state {
            Some(PagerState::Ready { next_request }) => {
                let request_future = self.client.request(&next_request);
                self.state = Some(PagerState::Pending {
                    request: next_request,
                    request_future,
                });
                self.poll_next(cx)
            }
            Some(PagerState::Pending {
                mut request,
                mut request_future,
            }) => {
                let response = match request_future.poll_unpin(cx) {
                    Poll::Pending => {
                        self.state = Some(PagerState::Pending {
                            request,
                            request_future,
                        });
                        return Poll::Pending;
                    }
                    Poll::Ready(res) => res.map_err(Error::Client)?.into_result()?,
                };
                let mut response: Vec<_> = response.into_iter().collect();
                if let Some(until) = response.last() {
                    request.set_until_id(until.item_id());
                    if let Some(since_id) = self.since_id.take() {
                        request.set_since_id(since_id.clone());
                        response.retain(|item| item.item_id() > since_id);
                    }
                    self.state = Some(PagerState::Ready {
                        next_request: request,
                    });
                }
                Poll::Ready(Some(Ok(response)))
            }
            None => Poll::Ready(None),
        }
    }
}

impl<'a, C, R> FusedStream for BackwardPager<'a, C, R>
where
    C: Client + ?Sized,
    R: PaginationRequest + Unpin,
    R::Response: IntoIterator<Item = R::Item>,
    <R::Item as PaginationItem>::Id: Clone + Unpin,
{
    fn is_terminated(&self) -> bool {
        self.state.is_none()
    }
}

impl<'a, C, R> Pager for BackwardPager<'a, C, R>
where
    C: Client + ?Sized,
    R: PaginationRequest + Unpin,
    R::Response: IntoIterator<Item = R::Item>,
    <R::Item as PaginationItem>::Id: Clone + Unpin,
{
    type Content = R::Item;
    type Client = C;
}

pub(crate) struct ForwardPager<'a, C: Client + ?Sized, R: Request> {
    client: &'a C,
    state: Option<PagerState<'a, C, R>>,
}

impl<'a, C: Client + ?Sized, R: PaginationRequest> ForwardPager<'a, C, R> {
    pub(crate) fn new(client: &'a C, request: R) -> Self {
        ForwardPager {
            client,
            state: Some(PagerState::Ready {
                next_request: request,
            }),
        }
    }
}

impl<'a, C, R> Stream for ForwardPager<'a, C, R>
where
    C: Client + ?Sized,
    R: PaginationRequest + Unpin,
    R::Response: IntoIterator<Item = R::Item>,
{
    type Item = Result<Vec<R::Item>, Error<C::Error>>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let state = self.state.take();
        match state {
            Some(PagerState::Ready { next_request }) => {
                let request_future = self.client.request(&next_request);
                self.state = Some(PagerState::Pending {
                    request: next_request,
                    request_future,
                });
                self.poll_next(cx)
            }
            Some(PagerState::Pending {
                mut request,
                mut request_future,
            }) => {
                let response = match request_future.poll_unpin(cx) {
                    Poll::Pending => {
                        self.state = Some(PagerState::Pending {
                            request,
                            request_future,
                        });
                        return Poll::Pending;
                    }
                    Poll::Ready(res) => res.map_err(Error::Client)?.into_result()?,
                };
                let response: Vec<_> = response.into_iter().collect();
                if let Some(since) = response.last() {
                    request.set_since_id(since.item_id());
                    self.state = Some(PagerState::Ready {
                        next_request: request,
                    });
                }
                Poll::Ready(Some(Ok(response)))
            }
            None => Poll::Ready(None),
        }
    }
}

impl<'a, C, R> FusedStream for ForwardPager<'a, C, R>
where
    C: Client + ?Sized,
    R: PaginationRequest + Unpin,
    R::Response: IntoIterator<Item = R::Item>,
{
    fn is_terminated(&self) -> bool {
        self.state.is_none()
    }
}

impl<'a, C, R> Pager for ForwardPager<'a, C, R>
where
    C: Client + ?Sized,
    R: PaginationRequest + Unpin,
    R::Response: IntoIterator<Item = R::Item>,
{
    type Content = R::Item;
    type Client = C;
}

pub(crate) struct OffsetPager<'a, C: Client + ?Sized, R: Request> {
    client: &'a C,
    state: Option<PagerState<'a, C, R>>,
    total_count: u64,
}

impl<'a, C: Client + ?Sized, R: OffsetPaginationRequest> OffsetPager<'a, C, R> {
    pub(crate) fn new(client: &'a C, request: R) -> Self {
        OffsetPager {
            client,
            state: Some(PagerState::Ready {
                next_request: request,
            }),
            total_count: 0,
        }
    }
}

impl<'a, C, R> Stream for OffsetPager<'a, C, R>
where
    C: Client + ?Sized,
    R: OffsetPaginationRequest + Unpin,
    R::Response: IntoIterator<Item = R::Item>,
{
    type Item = Result<Vec<R::Item>, Error<C::Error>>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let state = self.state.take();
        match state {
            Some(PagerState::Ready { next_request }) => {
                let request_future = self.client.request(&next_request);
                self.state = Some(PagerState::Pending {
                    request: next_request,
                    request_future,
                });
                self.poll_next(cx)
            }
            Some(PagerState::Pending {
                mut request,
                mut request_future,
            }) => {
                let response = match request_future.poll_unpin(cx) {
                    Poll::Pending => {
                        self.state = Some(PagerState::Pending {
                            request,
                            request_future,
                        });
                        return Poll::Pending;
                    }
                    Poll::Ready(res) => res.map_err(Error::Client)?.into_result()?,
                };
                let response: Vec<_> = response.into_iter().collect();
                if !response.is_empty() {
                    self.total_count += response.len() as u64;
                    request.set_offset(self.total_count);
                    self.state = Some(PagerState::Ready {
                        next_request: request,
                    });
                }
                Poll::Ready(Some(Ok(response)))
            }
            None => Poll::Ready(None),
        }
    }
}

impl<'a, C, R> FusedStream for OffsetPager<'a, C, R>
where
    C: Client + ?Sized,
    R: OffsetPaginationRequest + Unpin,
    R::Response: IntoIterator<Item = R::Item>,
{
    fn is_terminated(&self) -> bool {
        self.state.is_none()
    }
}

impl<'a, C, R> Pager for OffsetPager<'a, C, R>
where
    C: Client + ?Sized,
    R: OffsetPaginationRequest + Unpin,
    R::Response: IntoIterator<Item = R::Item>,
{
    type Content = R::Item;
    type Client = C;
}

/// A stream of pages..
pub trait Pager:
    Stream<
    Item = Result<Vec<<Self as Pager>::Content>, Error<<<Self as Pager>::Client as Client>::Error>>,
>
{
    /// Values yielded by the pager.
    type Content;
    /// [`Client`][`misskey_core::Client`] type used in the pager.
    type Client: Client + ?Sized;
}

impl<P: Pager + Unpin + ?Sized> Pager for &mut P {
    type Content = P::Content;
    type Client = P::Client;
}

impl<P: Pager + Unpin + ?Sized> Pager for Box<P> {
    type Content = P::Content;
    type Client = P::Client;
}

impl<P> Pager for Pin<P>
where
    P: DerefMut + Unpin,
    <P as Deref>::Target: Pager,
{
    type Content = <<P as Deref>::Target as Pager>::Content;
    type Client = <<P as Deref>::Target as Pager>::Client;
}

impl<S, F, T> Pager for futures::stream::MapOk<S, F>
where
    S: Pager,
    F: FnMut(Vec<<S as Pager>::Content>) -> Vec<T>,
{
    type Content = T;
    type Client = <S as Pager>::Client;
}

/// An owned dynamically typed [`Pager`].
pub type BoxPager<'a, C, T> = Pin<
    Box<
        dyn Pager<Content = T, Client = C, Item = Result<Vec<T>, Error<<C as Client>::Error>>>
            + 'a
            + Send,
    >,
>;

/// A stream of elements in [`Pager`].
pub struct PagerStream<P: Pager> {
    pager: P,
    buffer: VecDeque<P::Content>,
}

impl<P: Pager> PagerStream<P> {
    pub(crate) fn new(pager: P) -> Self {
        PagerStream {
            pager,
            buffer: VecDeque::new(),
        }
    }

    /// Returns the inner pager.
    pub fn into_inner(self) -> P {
        self.pager
    }
}

impl<P> Stream for PagerStream<P>
where
    P: Pager + Unpin,
    P::Content: Unpin + std::fmt::Debug,
{
    type Item = Result<P::Content, Error<<P::Client as Client>::Error>>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        if let Some(item) = self.buffer.pop_front() {
            Poll::Ready(Some(Ok(item)))
        } else if let Some(page) = futures::ready!(self.pager.poll_next_unpin(cx)) {
            self.buffer.extend(page?);
            Poll::Ready(self.buffer.pop_front().map(Ok))
        } else {
            Poll::Ready(None)
        }
    }
}

impl<P> FusedStream for PagerStream<P>
where
    P: Pager + FusedStream + Unpin,
    P::Content: Unpin + std::fmt::Debug,
{
    fn is_terminated(&self) -> bool {
        self.buffer.is_empty() && self.pager.is_terminated()
    }
}
