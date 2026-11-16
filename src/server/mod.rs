mod tls;

#[cfg(feature = "docs")]
mod docs;
#[cfg(feature = "docs")]
pub use docs::*;

use std::path::PathBuf;
use std::time::Duration;


use tokio::signal;
use salvo::prelude::*;
use salvo::catcher::Catcher;






/// # Setup Server
/// ### http3 **(quic)**, http2 / http1 **(tcp)**
pub struct WebServer {
    tls: PathBuf,
    router: Router,
    catcher: Catcher,

    #[cfg(feature = "docs")]
    oapi: Option<Router>,

    #[cfg(feature = "docs")]
    oapi_docs: Option<DocsStore>,

    timeout: Option<Duration>,
    addr: (&'static str, u16),
    subject_alt_names: Option<Vec<&'static str>>,
}

impl WebServer {
    
    pub fn new(
        router: Router,
        catcher: Catcher,
        timeout: Option<Duration>,
        addr: (&'static str, u16),
        tls: impl AsRef<std::path::Path>,
        subject_alt_names: Option<Vec<&'static str>>,
    ) -> Self {
        Self {
            addr,
            timeout,
            router: router,
            catcher: catcher,
            subject_alt_names,
            tls: tls.as_ref().to_path_buf(),

            #[cfg(feature = "docs")]
            oapi: None,
            
            #[cfg(feature = "docs")]
            oapi_docs: None,
        }
    }
    
    #[cfg(feature = "docs")]
    pub fn add_oapi(&mut self, title: &str, version: &str, path: &str, router: Router) {

        if self.oapi.is_none() {
            self.oapi = Some(Router::new());
        }

        if self.oapi_docs.is_none() {
            self.oapi_docs = Some(DocsStore::new());
        }

        let docs = OpenApi::new(title, version).merge_router(&router);

        self.oapi_docs
            .as_mut()
            .unwrap()
            .add(path, docs.clone());

        self.oapi
            .as_mut()
            .unwrap()
            .routers_mut()
            .push(router.unshift(docs.into_router(path)));

    }

    pub async fn run(self) -> Result<(), ServerError> {

        let tls_path = self.tls.as_path();
        let config = tls::create_rustls_config(tls_path, self.subject_alt_names)?;
        let listener = TcpListener::new(self.addr).rustls(config.clone());
        let acceptor = QuinnListener::new(config.build_quinn_config().unwrap(), self.addr).join(listener).bind().await;


        #[cfg(feature = "docs")]
        let router = match self.oapi {
            Some(oapi) => {
                let mut r = self.router.hoop(affix_state::inject(std::sync::Arc::new(parking_lot::RwLock::new(self.oapi_docs.unwrap()))));
                r.routers_mut().push(oapi);
                r
            },
            None => {
                self.router
            }
        };

        #[cfg(not(feature = "docs"))]
        let router = self.router;

        let service = Service::new(router).hoop(ForceHttps::new().https_port(self.addr.1));
        let server = Server::new(acceptor);
        let handle = server.handle();

        tokio::spawn(async move {

            let ctrl_c = async {
                signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
            };

            #[cfg(unix)]
            let terminate = async {
                signal::unix::signal(signal::unix::SignalKind::terminate()).expect("failed to install signal handler").recv().await;
            };

            #[cfg(windows)]
            let terminate = async {
                signal::windows::ctrl_c().expect("failed to install signal handler").recv().await;
            };

            tokio::select! {
                _ = ctrl_c => tracing::info!("ctrl_c signal received"),
                _ = terminate => tracing::info!("terminate signal received"),
            }

            handle.stop_graceful(self.timeout);

        });

        Ok(server.serve(service.catcher(self.catcher)).await)

    }

}






#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Fmt(#[from] std::fmt::Error),
    #[error(transparent)]
    SalvoError(#[from] salvo::Error),
    #[error(transparent)]
    Join(#[from] tokio::task::JoinError),
    
    #[error(transparent)]
    TlsServerError(#[from] tls::TlsServerError),
    
    #[cfg(feature = "docs")]
    #[error(transparent)]
    DocsError(#[from] docs::DocsError),
}