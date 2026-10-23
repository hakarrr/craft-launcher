//! The shipped `Http` agents refuse plain-HTTP URLs before connecting.
//!
//! This is an integration test, not a unit test inside the crate, so the library builds without
//! `cfg(test)` and `Http::new()` produces the production agents with `https_only` set. The
//! crate's own unit tests allow plain HTTP so their test server can speak loopback HTTP, which
//! means they can only pin ureq's behavior, not the agents the launcher ships.

#![cfg(not(feature = "local-mirror"))]

use std::net::TcpListener;

use artcraft_launcher_native::http::Http;

#[test]
fn production_agents_refuse_plain_http_before_connecting() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind test server");
    listener.set_nonblocking(true).expect("nonblocking listener");
    let url = format!("http://{}/releases/latest", listener.local_addr().expect("listener address"));

    let http = Http::new().expect("Http::new");
    assert!(http.get_api(&url, None).is_err(), "plain-HTTP API request succeeded");
    assert!(http.get_feed(&url).is_err(), "plain-HTTP feed request succeeded");
    assert!(http.get_text(&url).is_err(), "plain-HTTP GET succeeded");
    assert!(http.redirect_target(&url).is_err(), "plain-HTTP redirect probe succeeded");

    assert!(listener.accept().is_err(), "a plain-HTTP request reached the server");
}
