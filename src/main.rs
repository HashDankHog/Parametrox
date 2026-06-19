// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
use parametrox::run;
use parametrox::bench;
fn main() {
  for _ in 0..100{
    bench();
  }
  run();
}
