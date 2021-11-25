#![deny(clippy::all)]

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::ready;
use http::{header::HeaderMap, Request, Response};
use pin_project::pin_project;
use tower_layer::Layer;
use tower_service::Service;

#[pin_project]
pub struct ResponseFuture<F> {
    #[pin]
    default_headers: HeaderMap,
    #[pin]
    future: F,
}
impl<F, ResponseBody, E> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response<ResponseBody>, E>>,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let this = self.project();
        let mut res = ready!(this.future.poll(cx)?);
        let headers = res.headers_mut();

        for (name, value) in this.default_headers.iter() {
            if !headers.contains_key(name) {
                headers.insert(name, value.clone());
            }
        }

        Poll::Ready(Ok(res))
    }
}

#[derive(Clone)]
pub struct DefaultHeaders<S> {
    default_headers: HeaderMap,
    inner: S,
}
impl<S> DefaultHeaders<S> {}
impl<RequestBody, ResponseBody, S> Service<Request<RequestBody>> for DefaultHeaders<S>
where
    S: Service<Request<RequestBody>, Response = Response<ResponseBody>>,
{
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;
    type Response = S::Response;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<RequestBody>) -> Self::Future {
        ResponseFuture {
            // TODO: juggle lifetimes and pass this in as a borrow
            default_headers: self.default_headers.clone(),
            future: self.inner.call(req),
        }
    }
}

#[derive(Clone)]
pub struct DefaultHeadersLayer {
    default_headers: HeaderMap,
}
impl DefaultHeadersLayer {
    pub fn new(default_headers: HeaderMap) -> Self {
        Self { default_headers }
    }
}
impl<S> Layer<S> for DefaultHeadersLayer {
    type Service = DefaultHeaders<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Self::Service {
            // TODO: juggle lifetimes and pass this in as a borrow
            default_headers: self.default_headers.clone(),
            inner,
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{
            header::{HeaderValue, X_FRAME_OPTIONS},
            Request, StatusCode,
        },
        routing::{get, Router},
    };
    use tower::ServiceExt;

    use super::*;

    #[tokio::test]
    async fn test_headers_when_missing() {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(X_FRAME_OPTIONS, HeaderValue::from_static("deny"));

        let app = Router::new()
            .route("/", get(|| async { "hello, world!" }))
            .layer(DefaultHeadersLayer::new(default_headers));

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let headers = response.headers();
        assert_eq!(headers["x-frame-options"], "deny");

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"hello, world!");
    }

    #[tokio::test]
    async fn test_headers_when_already_set_by_handler() {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(X_FRAME_OPTIONS, HeaderValue::from_static("deny"));

        let app = Router::new()
            .route(
                "/",
                get(|| async {
                    let mut headers = HeaderMap::new();
                    headers.insert("x-frame-options", HeaderValue::from_static("sameorigin"));
                    (headers, "hello, world!")
                }),
            )
            .layer(DefaultHeadersLayer::new(default_headers));

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let headers = response.headers();
        assert_eq!(headers["x-frame-options"], "sameorigin");

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"hello, world!");
    }
}
