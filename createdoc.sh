#!/bin/bash

#!/bin/bash
if [ "$TRAVIS_BRANCH" == "master" ]; then
  echo $TRAVIS_RUST_VERSION
  cargo doc --target-dir public
fi