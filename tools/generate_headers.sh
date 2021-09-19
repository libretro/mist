#!/bin/bash

cbindgen --crate mist-library --lang c --output "$(dirname $0)/../include/mist.h"