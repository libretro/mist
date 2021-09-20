#!/bin/bash

cbindgen --crate mist --lang c --output "$(dirname $0)/../include/mist.h"