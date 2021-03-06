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
use std::time::{Duration, Instant};

use crate::Error;

use futures::{
    future::{BoxFuture, FutureExt},
    stream::{FusedStream, Stream, StreamExt},
};
use futures_timer::Delay;
use misskey_api::{OffsetPaginationRequest, PaginationItem, PaginationRequest};
use misskey_core::model::ApiResult;
use misskey_core::{Client, Request};

const DEFAULT_PAGE_SIZE: u8 = 30;

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
        mut request: R,
    ) -> Self {
        request.set_limit(DEFAULT_PAGE_SIZE);
        BackwardPager {
            client,
            since_id,
            state: Some(PagerState::Ready {
                next_request: request,
            }),
        }
    }

    pub(crate) fn new(client: &'a C, request: R) -> Self {
        BackwardPager::with_since_id(client, None, request)
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

    fn set_page_size(mut self: Pin<&mut Self>, size: u8) {
        match self.state.as_mut() {
            Some(PagerState::Ready { next_request, .. }) => next_request.set_limit(size),
            Some(PagerState::Pending { request, .. }) => request.set_limit(size),
            None => {}
        }
    }
}

pub(crate) struct ForwardPager<'a, C: Client + ?Sized, R: Request> {
    client: &'a C,
    state: Option<PagerState<'a, C, R>>,
}

impl<'a, C: Client + ?Sized, R: PaginationRequest> ForwardPager<'a, C, R> {
    pub(crate) fn new(client: &'a C, mut request: R) -> Self {
        request.set_limit(DEFAULT_PAGE_SIZE);
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

    fn set_page_size(mut self: Pin<&mut Self>, size: u8) {
        match self.state.as_mut() {
            Some(PagerState::Ready { next_request, .. }) => next_request.set_limit(size),
            Some(PagerState::Pending { request, .. }) => request.set_limit(size),
            None => {}
        }
    }
}

pub(crate) struct OffsetPager<'a, C: Client + ?Sized, R: Request> {
    client: &'a C,
    state: Option<PagerState<'a, C, R>>,
    total_count: u64,
}

impl<'a, C: Client + ?Sized, R: OffsetPaginationRequest> OffsetPager<'a, C, R> {
    pub(crate) fn new(client: &'a C, mut request: R) -> Self {
        request.set_limit(DEFAULT_PAGE_SIZE);
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

    fn set_page_size(mut self: Pin<&mut Self>, size: u8) {
        match self.state.as_mut() {
            Some(PagerState::Ready { next_request, .. }) => next_request.set_limit(size),
            Some(PagerState::Pending { request, .. }) => request.set_limit(size),
            None => {}
        }
    }
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

    /// Sets the number of items to be fetched at once.
    fn set_page_size(self: Pin<&mut Self>, size: u8);
}

impl<P: Pager + Unpin + ?Sized> Pager for &mut P {
    type Content = P::Content;
    type Client = P::Client;

    fn set_page_size(mut self: Pin<&mut Self>, size: u8) {
        P::set_page_size(Pin::new(&mut **self), size)
    }
}

impl<P: Pager + Unpin + ?Sized> Pager for Box<P> {
    type Content = P::Content;
    type Client = P::Client;

    fn set_page_size(mut self: Pin<&mut Self>, size: u8) {
        P::set_page_size(Pin::new(&mut **self), size)
    }
}

impl<P> Pager for Pin<P>
where
    P: DerefMut + Unpin,
    <P as Deref>::Target: Pager,
{
    type Content = <<P as Deref>::Target as Pager>::Content;
    type Client = <<P as Deref>::Target as Pager>::Client;

    fn set_page_size(self: Pin<&mut Self>, size: u8) {
        <<P as Deref>::Target as Pager>::set_page_size(self.get_mut().as_mut(), size)
    }
}

impl<S, F, T> Pager for futures::stream::MapOk<S, F>
where
    S: Pager + Unpin,
    F: FnMut(Vec<<S as Pager>::Content>) -> Vec<T>,
{
    type Content = T;
    type Client = <S as Pager>::Client;

    fn set_page_size(mut self: Pin<&mut Self>, size: u8) {
        <S as Pager>::set_page_size(Pin::new(&mut *(*self).get_mut()), size)
    }
}

/// An owned dynamically typed [`Pager`].
pub type BoxPager<'a, C, T> = Pin<
    Box<
        dyn Pager<Content = T, Client = C, Item = Result<Vec<T>, Error<<C as Client>::Error>>>
            + 'a
            + Send,
    >,
>;

enum PagerStreamState<P: Pager> {
    Ready {
        item: P::Content,
        buffer: VecDeque<P::Content>,
        last_fetch: Instant,
    },
    Fetch,
    Wait(Delay),
}

/// A stream of elements in [`Pager`].
pub struct PagerStream<P: Pager> {
    pager: P,
    minimum_interval: Duration,
    state: Option<PagerStreamState<P>>,
}

impl<P: Pager> PagerStream<P> {
    pub(crate) fn new(pager: P) -> Self {
        PagerStream {
            pager,
            minimum_interval: Duration::new(0, 0),
            state: Some(PagerStreamState::Fetch),
        }
    }

    /// Sets the number of items to be fetched at once by the inner pager.
    pub fn set_page_size(&mut self, size: u8)
    where
        P: Unpin,
    {
        Pin::new(&mut self.pager).set_page_size(size);
    }

    /// Sets the minimum time interval between pagination.
    ///
    /// It is recommended to set this to reduce the load on the
    /// server if you expect a lot of pagination.
    pub fn set_interval(&mut self, minimum_interval: Duration) {
        self.minimum_interval = minimum_interval;
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
        let state = self.state.take();
        match state {
            Some(PagerStreamState::Ready {
                item,
                mut buffer,
                last_fetch,
            }) => {
                if let Some(next) = buffer.pop_front() {
                    self.state = Some(PagerStreamState::Ready {
                        item: next,
                        buffer,
                        last_fetch,
                    });
                } else {
                    let until = last_fetch + self.minimum_interval;
                    if let Some(duration) = until.checked_duration_since(Instant::now()) {
                        self.state = Some(PagerStreamState::Wait(Delay::new(duration)));
                    } else {
                        self.state = Some(PagerStreamState::Fetch);
                    }
                }
                Poll::Ready(Some(Ok(item)))
            }
            Some(PagerStreamState::Fetch) => match self.pager.poll_next_unpin(cx) {
                Poll::Pending => {
                    self.state = Some(PagerStreamState::Fetch);
                    Poll::Pending
                }
                Poll::Ready(None) => Poll::Ready(None),
                Poll::Ready(Some(Err(err))) => Poll::Ready(Some(Err(err))),
                Poll::Ready(Some(Ok(page))) => {
                    let mut buffer: VecDeque<_> = page.into();
                    if let Some(item) = buffer.pop_front() {
                        self.state = Some(PagerStreamState::Ready {
                            item,
                            buffer,
                            last_fetch: Instant::now(),
                        });
                        self.poll_next(cx)
                    } else {
                        Poll::Ready(None)
                    }
                }
            },
            Some(PagerStreamState::Wait(mut delay)) => match delay.poll_unpin(cx) {
                Poll::Pending => {
                    self.state = Some(PagerStreamState::Wait(delay));
                    Poll::Pending
                }
                Poll::Ready(()) => {
                    self.state = Some(PagerStreamState::Fetch);
                    self.poll_next(cx)
                }
            },
            None => Poll::Ready(None),
        }
    }
}

impl<P> FusedStream for PagerStream<P>
where
    P: Pager + FusedStream + Unpin,
    P::Content: Unpin + std::fmt::Debug,
{
    fn is_terminated(&self) -> bool {
        self.state.is_none()
    }
}
