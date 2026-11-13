use salvo::conn::rustls::{Keycert, RustlsConfig};

use rcgen::{generate_simple_self_signed, CertifiedKey};


static DEFAULT_HOSTS: &[&str] = &[ "localhost", "127.0.0.1", "0.0.0.0", "::1", "::" ];

pub fn generate_or_load_self_signed(
    path: impl AsRef<std::path::Path> + Copy,
    subject_alt_names: Option<Vec<&str>>
) -> Result<(Vec<u8>, Vec<u8>), TlsServerError> {
    
    if !path.as_ref().exists() {
        std::fs::create_dir_all(path)?;
    }

    let key = path.as_ref().join("key.pem");
    let certs = path.as_ref().join("certs.pem");

    if !key.exists() && !certs.exists() {
        let CertifiedKey { cert, signing_key } = generate_simple_self_signed(
            subject_alt_names
                .unwrap_or(DEFAULT_HOSTS.to_vec())
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
        ).unwrap();
        std::fs::write(&key, signing_key.serialize_pem())?;
        std::fs::write(&certs, cert.pem())?;
    }

    Ok((std::fs::read(certs)?, std::fs::read(key)?))

}

pub fn create_rustls_config(
    path: impl AsRef<std::path::Path> + Copy,
    subject_alt_names: Option<Vec<&str>>
) -> Result<RustlsConfig, TlsServerError> {

    let tls = generate_or_load_self_signed(path.as_ref(), subject_alt_names)?;

    Ok(RustlsConfig::new(Keycert::new().cert(tls.0).key(tls.1)))

}



#[derive(Debug, thiserror::Error)]
pub enum TlsServerError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Fmt(#[from] std::fmt::Error),
    #[error(transparent)]
    SalvoError(#[from] salvo::Error),
    #[error(transparent)]
    Join(#[from] tokio::task::JoinError),
}