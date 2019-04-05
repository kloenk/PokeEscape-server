#!/bin/bash

#!/bin/bash
if [ "$TRAVIS_BRANCH" == "master" ]; then
  cargo doc --target-dir public
  cd public
fi