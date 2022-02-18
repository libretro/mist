#!/bin/bash

cargo run --bin generate_headers -- "$(dirname $0)/../"
(cd $(dirname $0)/../ && cbindgen --config cbindgen.toml --crate mist --output "$(dirname $0)/../include/mist.h")
