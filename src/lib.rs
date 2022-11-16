use std::path::PathBuf;
use std::str::FromStr;

use axum_server::tls_rustls::RustlsConfig;

pub async fn get_tls_config()  {
    let testval1 = PathBuf::from_str("/test").unwrap();
    let testval2 = PathBuf::from_str("/test").unwrap();

    let _ = RustlsConfig::from_pem_file(testval1.to_owned(), testval2.clone()).await;
}
