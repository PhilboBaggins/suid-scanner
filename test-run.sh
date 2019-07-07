#!/bin/sh

cd "$(dirname -- "$0")"

exec cargo run -- {/,/usr/,/usr/local/}{bin,sbin}
