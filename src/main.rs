// CLIを表示しない（アタッチされないので標準出力は出ない）
// #![windows_subsystem = "windows"]
mod action;
mod state;
mod app;

use action::*;
use state::*;

static RESOURCE: include_dir::Dir = include_dir::include_dir!("$CARGO_MANIFEST_DIR/src/resource");

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  #[arg(short, long, default_value_t = String::from("http://wry.localhost/"))]
  start: String,
}

fn main() -> anyhow::Result<()> {
  let state = State::default();
  let args = Args::parse();

  let event_loop = winit::event_loop::EventLoop::<Action>::with_user_event().build()?;
  let mut app = app::App::new(args.start.as_str(), state, &event_loop);
  event_loop.run_app(&mut app).unwrap();
  
  Ok(())
}
