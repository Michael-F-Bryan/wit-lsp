use std::{any::Any, future::Future, panic::AssertUnwindSafe, pin::Pin};

use futures::FutureExt;
use tower_lsp::LspService;
use tower_service::Service;
use tracing::Instrument;
use uuid::Uuid;

use crate::server::LanguageServer;

pub(crate) fn wrap(service: LspService<LanguageServer>) -> impl LanguageServerService {
    LoggingService(CatchPanic(service))
}

/// A trait alias for a [`tower_service::Service`] which matches the
/// API required by [`tower_lsp::Server::serve()`].
pub trait LanguageServerService
where
    Self: Service<
            tower_lsp::jsonrpc::Request,
            Response = Option<tower_lsp::jsonrpc::Response>,
            Error = CatchPanicError,
            Future = Pin<
                Box<
                    dyn Future<
                            Output = Result<Option<tower_lsp::jsonrpc::Response>, CatchPanicError>,
                        > + Send
                        + 'static,
                >,
            >,
        > + Send
        + 'static,
{
}

impl<T> LanguageServerService for T where
    T: Service<
            tower_lsp::jsonrpc::Request,
            Response = Option<tower_lsp::jsonrpc::Response>,
            Error = CatchPanicError,
            Future = Pin<
                Box<
                    dyn Future<
                            Output = Result<Option<tower_lsp::jsonrpc::Response>, CatchPanicError>,
                        > + Send
                        + 'static,
                >,
            >,
        > + Send
        + 'static
{
}

#[derive(Debug, Clone)]
pub(crate) struct LoggingService<S>(pub S);

impl<S> Service<tower_lsp::jsonrpc::Request> for LoggingService<S>
where
    S: Service<tower_lsp::jsonrpc::Request, Response = Option<tower_lsp::jsonrpc::Response>>,
    S::Error: std::error::Error + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    fn call(&mut self, req: tower_lsp::jsonrpc::Request) -> Self::Future {
        let id = match req.id() {
            Some(tower_lsp::jsonrpc::Id::Number(n)) => n.to_string(),
            Some(tower_lsp::jsonrpc::Id::String(s)) => s.clone(),
            None | Some(tower_lsp::jsonrpc::Id::Null) => Uuid::new_v4().to_string(),
        };
        let method = req.method().to_string();

        let fut = self.0.call(req);
        let fut = async move {
            let ret = fut.await;

            match ret.as_ref() {
                Ok(r) => {
                    if let Some(err) = r.as_ref().and_then(|r| r.error()) {
                        tracing::debug!(error = err as &dyn std::error::Error, "Returned an error",);
                    }
                }
                Err(err) => {
                    tracing::error!(
                        error = err as &dyn std::error::Error,
                        "An error occurred while handling the request",
                    )
                }
            }

            ret
        };

        Box::pin(fut.instrument(tracing::debug_span!("request", %id, %method)))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CatchPanic<S>(pub S);

impl<S> Service<tower_lsp::jsonrpc::Request> for CatchPanic<S>
where
    S: Service<
        tower_lsp::jsonrpc::Request,
        Response = Option<tower_lsp::jsonrpc::Response>,
        Error = tower_lsp::ExitedError,
    >,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = CatchPanicError;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx).map_err(CatchPanicError::Exited)
    }

    fn call(&mut self, req: tower_lsp::jsonrpc::Request) -> Self::Future {
        let fut = AssertUnwindSafe(self.0.call(req)).catch_unwind();

        Box::pin(async move {
            match fut.await {
                Ok(result) => result.map_err(CatchPanicError::Exited),
                Err(payload) => Err(CatchPanicError::Panic(PanicMessage::new(payload))),
            }
        })
    }
}

/// Errors that may occur when you catch panics in a [`tower_service::Service`].
#[derive(Debug, Clone)]
pub enum CatchPanicError {
    Panic(PanicMessage),
    Exited(tower_lsp::ExitedError),
}

impl std::error::Error for CatchPanicError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CatchPanicError::Panic(p) => Some(p),
            CatchPanicError::Exited(e) => Some(e),
        }
    }
}

impl std::fmt::Display for CatchPanicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CatchPanicError::Panic(p) => write!(f, "{p}"),
            CatchPanicError::Exited(e) => write!(f, "{e}"),
        }
    }
}

/// A printable panic payload.
#[derive(Debug, Clone)]
pub struct PanicMessage {
    msg: Option<String>,
}

impl PanicMessage {
    fn new(payload: Box<dyn Any + Send>) -> Self {
        let msg = if let Some(msg) = payload.downcast_ref::<String>() {
            Some(msg.as_str())
        } else if let Some(&msg) = payload.downcast_ref::<&str>() {
            Some(msg)
        } else {
            None
        };

        PanicMessage {
            msg: msg.map(String::from),
        }
    }
}

impl std::error::Error for PanicMessage {}

impl std::fmt::Display for PanicMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = self.msg.as_deref().unwrap_or("<unknown>");
        write!(f, "A panic occurred while handling the request: {msg}")
    }
}
