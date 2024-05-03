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
        cargo install trunk@^0.19 # TODO 2024/05/03 0.20 cause empty page response
        ;;
    *)
        echo "do not install trunk. finish."
        exit 1
        ;;
    esac
fi

if ! (type clang >/dev/null 2>&1); then
    printf ">>> install clang? [y/N]"
    read -r ans
    case $ans in
    [Yy]*)
        sudo apt update
        sudo apt install -y clang
        ;;
    *)
        echo "do not install clang. finish."
        exit 1
        ;;
    esac
fi
