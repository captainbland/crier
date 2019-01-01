#!/bin/bash

# Sets up dependencies for selenium test suite


$(java -Dwebdriver.chrome.driver=/usr/bin/chromedriver -jar ~/selenium-server-standalone.jar) &
#uncomment for xvfb headless: (. xvfb-run -a java -Dwebdriver.chrome.driver=/usr/bin/chromedriver -jar ~/selenium-server-standalone.jar) &
xvproc=$!
selproc=$(lsof -t -i :4444)
export DATABASE_URL='postgres://root:password@127.0.0.1/criertest'
export RUST_BACKTRACE=1
diesel database reset
cargo run crier --features="debug" &
crierproc=$!

sleep 2
cargo test -- --test-threads=1
echo "test test test"
#tidy up

kill $xvproc
kill $selproc
kill $crierproc