#! /usr/bin/env bash

wrk -t12 -c100 -d60s http://localhost:5000/bucket/default/doc/foo

