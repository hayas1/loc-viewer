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

if ! (type graphql-client >/dev/null 2>&1); then
    printf ">>> install graphql_client_cli? [y/N]"
    read -r ans
    case $ans in
    [Yy]*)
        cargo install graphql_client_cli
        ;;
    *)
        echo "do not install graphql_client_cli. finish."
        exit 1
        ;;
    esac
fi
