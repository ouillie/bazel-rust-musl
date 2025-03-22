use openssl::ssl::{SslMethod, SslConnector};

fn main() {
    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
}
