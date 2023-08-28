#!/bin/bash
set -e

( #
  cd www/src/extras
  [ ! -e "extra-vue-ui" ] && git clone https://github.com/juliendecharentenay/extra-vue-ui.git
)

( #
  cd rust/extras
  [ ! -e "extra-rust-wasm" ] && git clone https://github.com/juliendecharentenay/extra-rust-wasm.git
)
