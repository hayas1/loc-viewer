#! /bin/sh -e

scripts=$(dirname "$(realpath "$0")")
"$scripts"/setup.sh

repo=$(dirname "$(dirname "$(realpath "$0")")")

trunk serve --dist "$repo"/public --port 8080
