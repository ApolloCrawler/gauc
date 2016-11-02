#! /usr/bin/env bash

HOST=localhost
PORT=5000

wrk -t12 -c100 -d60s http://$HOST:$PORT/bucket/default/doc/foo

