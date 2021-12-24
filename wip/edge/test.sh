#!/bin/bash

cargo run --bin idea_graph2 > graph.dot; dot -Tjpeg graph.dot > graph.jpeg; open graph.jpeg
