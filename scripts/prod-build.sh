#! /bin/sh -e

scripts=$(dirname "$(realpath "$0")")
"$scripts"/setup.sh

repo=$(dirname "$(dirname "$(realpath "$0")")")

PUBLIC_URL="${PUBLIC_URL:-"/tokei-toukei/"}"
trunk build --release --dist "$repo"/target/public --public-url "$PUBLIC_URL"
