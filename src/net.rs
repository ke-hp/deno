use errors::DenoResult;
use hyper;
use hyper::rt::{Future, Stream};
use hyper::{Client, Uri};
use hyper_rustls;

// The CodeFetch message is used to load HTTP javascript resources and expects a
// synchronous response, this utility method supports that.
pub fn fetch_sync_string(module_name: &str) -> DenoResult<String> {
  let url = module_name.parse::<Uri>().unwrap();
  let https = hyper_rustls::HttpsConnector::new(4);
  let client: Client<_, hyper::Body> = Client::builder().build(https);
  /*
  let body = tokio::block_on(
    client
      .get(url)
      .and_then(|response| response.into_body().concat2())
  )?;
  */
  let body = client
    .get(url)
    .and_then(|response| response.into_body().concat2())
    .wait()?;

  Ok(String::from_utf8(body.to_vec()).unwrap())
}

#[test]
fn test_fetch_sync_string() {
  // Relies on external http server. See tools/http_server.py
  use futures;
  use tokio;

  // Create a runtime so tokio stuff can execute.
  let mut rt = tokio::runtime::Runtime::new().unwrap();

  rt.block_on(futures::future::lazy(|| -> DenoResult<()> {
    let p = fetch_sync_string("http://localhost:4545/package.json")?;
    println!("package.json len {}", p.len());
    assert!(p.len() > 1);
    Ok(())
  })).unwrap();

  rt.shutdown_now().wait().unwrap();
}
