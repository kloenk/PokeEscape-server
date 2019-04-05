#!/bin/bash

#!/bin/bash
if [ "$TRAVIS_BRANCH" == "master" ]; then
  if [ "$TRAVIS_RUST_VERSION" == "stable" ]; then
    cargo doc --target-dir public
    cat <<EOF > ./public/index.html 
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8" lang="en"/>
        <meta http-equiv="refresh" content="0; URL='/pokemon_escape_server/doc/pokemon_escape_server/'" />
    </head>
</html>
EOF
  fi
fi