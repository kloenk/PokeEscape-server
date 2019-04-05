#!/bin/bash

#!/bin/bash
if [ "$TRAVIS_BRANCH" == "master" ]; then
  if [ "$TRAVIS_RUST_VERSION" == "stable"]; then
    cargo doc --target-dir public
  fi
fi