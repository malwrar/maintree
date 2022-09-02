#!/bin/bash

cargo run --bin tracking_test_1 | dot -Tpng | display
