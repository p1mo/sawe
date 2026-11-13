mod tls;



use tokio::signal;




use salvo::prelude::*;
use salvo::catcher::Catcher;
use salvo::server::ServerHandle;



pub async fn http3_server(
    r: Router,
    c: Catcher,
    addr: (&'static str, u16),
    path: impl AsRef<std::path::Path>
) -> Result<(), ServerError> {

    let config = tls::create_rustls_config(path.as_ref(), None)?;

    let listener = TcpListener::new(addr).rustls(config.clone());

    let acceptor = QuinnListener::new(config.build_quinn_config().unwrap(), addr).join(listener).bind().await;

    let service = Service::new(r).hoop(ForceHttps::new().https_port(addr.1));

    let server = Server::new(acceptor);

    let handle = server.handle();

    tokio::spawn(listen_shutdown_signal(handle));

    Ok(server.serve(service.catcher(c)).await)

}



async fn listen_shutdown_signal(handle: ServerHandle) -> Result<(), ServerError> {
    
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

    handle.stop_graceful(None);

    Ok(())
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
}