#!/bin/bash

#!/bin/bash
if [ "$TRAVIS_BRANCH" == "master" ]; then
  if [ "$TRAVIS_RUST_VERSION" == "stable" ]; then
    cargo doc --document-private-items --target-dir public
    cat <<EOF > ./public/index.html 
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8" lang="en"/>
        <meta http-equiv="refresh" content="0; URL='/doc/poke_escape_server/'" />
    </head>
</html>
EOF
  fi
fi
