// Copyright 2018 the Deno authors. All rights reserved. MIT license.
extern crate flatbuffers;
extern crate futures;
extern crate hyper;
extern crate libc;
extern crate msg_rs as msg;
extern crate rand;
extern crate tempfile;
extern crate tokio;
extern crate tokio_executor;
extern crate url;
#[macro_use]
extern crate log;
extern crate dirs;
extern crate hyper_rustls;
extern crate remove_dir_all;
extern crate ring;

mod deno_dir;
mod errors;
mod flags;
mod fs;
pub mod handlers;
mod isolate;
mod libdeno;
mod net;
mod version;

use std::env;

static LOGGER: Logger = Logger;

struct Logger;

impl log::Log for Logger {
  fn enabled(&self, metadata: &log::Metadata) -> bool {
    metadata.level() <= log::max_level()
  }

  fn log(&self, record: &log::Record) {
    if self.enabled(record.metadata()) {
      println!("{} RS - {}", record.level(), record.args());
    }
  }
  fn flush(&self) {}
}

// Set the default executor so we can use tokio::spawn(). It's difficult to
// pass around mut references to the runtime, so using with_default is
// preferable. Ideally Tokio would provide this function.
fn tokio_init<F>(f: F)
where
  F: Fn(),
{
  let rt = tokio::runtime::Runtime::new().unwrap();
  let mut executor = rt.executor();
  let mut enter = tokio_executor::enter().expect("Multiple executors at once");
  tokio_executor::with_default(&mut executor, &mut enter, move |_enter| f());
}

fn main() {
  log::set_logger(&LOGGER).unwrap();
  let args = env::args().collect();
  let isolate = isolate::Isolate::new(args, handlers::msg_from_js);
  flags::process(&isolate.state.flags);
  tokio_init(|| {
    isolate
      .execute("deno_main.js", "denoMain();")
      .unwrap_or_else(|err| {
        error!("{}", err);
        std::process::exit(1);
      });
    isolate.event_loop();
  });
}
