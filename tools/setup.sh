#! /bin/sh -e

if (type rustup >/dev/null 2>&1); then
    rustup target add wasm32-unknown-unknown
else
    printf "rustup should be installed. finish."
    exit 1
fi

if ! (type trunk >/dev/null 2>&1); then
    printf ">>> install trunk? [y/N]"
    read -r ans
    case $ans in
    [Yy]*)
        cargo install trunk
        ;;
    *)
        echo "do not install trunk. finish."
        exit 1
        ;;
    esac
fi
