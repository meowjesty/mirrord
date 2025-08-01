use std::{
    pin::Pin,
    task::{Context, Poll},
    vec,
};

use bytes::Bytes;
use hyper::body::{Body, Frame, Incoming};

/// [`Body`] that consist of some first [`Frame`]s that were already received,
/// and the optional rest of the body.
pub struct RolledBackBody<B = Incoming> {
    pub head: vec::IntoIter<Frame<Bytes>>,
    pub tail: Option<B>,
}

impl<B> Body for RolledBackBody<B>
where
    B: Body<Data = Bytes, Error = hyper::Error> + Unpin,
{
    type Data = Bytes;
    type Error = hyper::Error;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let this = self.get_mut();

        if let Some(frame) = this.head.next() {
            return Poll::Ready(Some(Ok(frame)));
        }

        let Some(tail) = this.tail.as_mut() else {
            return Poll::Ready(None);
        };

        Pin::new(tail).poll_frame(cx)
    }
}
