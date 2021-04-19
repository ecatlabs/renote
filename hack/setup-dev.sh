#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

function install_rust_dependencies() {
  if [[ -z $(command -v cargo 2>/dev/null) ]]; then
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    source "$HOME"/.cargo/env
    cargo version
  fi

  export_statement="export PATH=\$HOME/.cargo/bin:\$PATH"
  if ! grep -Fxq "$export_statement" ~/.bashrc; then
    echo "$export_statement" >>"$HOME"/.bashrc
  fi

  if [[ -f "$HOME"/.cargo/env ]]; then
    source "$HOME"/.cargo/env
  fi
}

os=$(uname)
case $os in
"Linux")
  install_linux_dependencies
  ;;
"Darwin")
  install_macos_dependencies
  ;;
esac

install_rust_dependencies
