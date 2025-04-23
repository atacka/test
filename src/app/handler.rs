use winit::event_loop::ActiveEventLoop;
use wry::http::{ Request, Response };
use crate::{ Action, State };
use std::sync::{ Arc, Mutex };
use reqwest::blocking;
use reqwest::blocking::Client;
use serde_json::json;

trait ToData {
  fn to_kind(&self) -> Option<String>;
}
impl ToData for Request<Vec<u8>> {
  fn to_kind(&self) -> Option<String> {
    serde_json::from_slice::<serde_json::Value>(self.body())
      .ok()
      .and_then(|v| v.get("kind").and_then(|kind| kind.as_str().map(|k| k.to_string())) )
  }
}

pub fn user_event (state: &Arc<Mutex<State>>, event_loop: Option<&ActiveEventLoop>, event: Action) -> Option<Response<Vec<u8>>> {
  match event {
    Action::GET(request) if request.uri().path().starts_with("/resource/") => {
      let url = request.uri().path().trim_start_matches("/resource/");
      println!("{url}");
      let content = match crate::RESOURCE.get_file(url) {
        Some(n) => n.contents_utf8().unwrap(),
        _ => {
          println!("{} {}", "get_file err", url);
          ""
        },
      };
      let header = match url {
        n if n.ends_with(".js") => "text/javascript",
        n if n.ends_with(".html") => "text/html",
        n if n.ends_with(".css") => "text/css",
        _=> "text/plain"
      };
      Some(wry::http::Response::builder()
        .header(wry::http::header::CONTENT_TYPE, header)
        .body(content.to_string().as_bytes().to_vec())
        .unwrap())
    },
    Action::GET(_request) => {
      println!("http://wry.localhost/ -> http://127.0.0.1:8000/");
      let url = "http://127.0.0.1:8000/";
      let response = blocking::get(url).unwrap();
      let content_type = response
        .headers()
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("").to_string();
      let body = response.bytes().unwrap();

      Some(wry::http::Response::builder()
        .header(wry::http::header::CONTENT_TYPE, content_type)
        .body(body.to_vec())
        .unwrap())
    },
    Action::POST(request) if request.to_kind() == Some("count".to_string()) => {
      let data = serde_json::from_slice::<serde_json::Value>(request.body())
        .ok()
        .and_then(|v| v.get("data").and_then(|data| data.as_i64()))
        .map_or(0, |s| s as i32);
      let mut state_lock = state.lock().unwrap();
      state_lock.count = state_lock.count + data;
      println!("count : {state_lock:?}");
      Some(wry::http::Response::builder()
        .header(wry::http::header::CONTENT_TYPE, "text/plain")
        .body(Vec::new())
        .unwrap())
    },
    Action::POST(_request) => {
      println!("POST unknown");
      Some(wry::http::Response::builder()
        .header(wry::http::header::CONTENT_TYPE, "text/plain")
        .body(Vec::new())
        .unwrap())
    },
    Action::Count(n) => {
      let mut state_lock = state.lock().unwrap();
      state_lock.count = state_lock.count + n;
      println!("count : {state_lock:?}");
      
      // JSONデータの作成
      let payload = json!({
        "kind": "cnt",
        "age": 30
      });
  
      let client = Client::new();

      // JSONデータの作成
      let _response = client
        .post("http://127.0.0.1:8000/")
        .json(&payload)
        .send().unwrap();

      None
    },
    Action::Close => {
      if let Some(e) = event_loop {
        e.exit();
      }
      None
    }
    Action::Unknown => { None }
  }
}



