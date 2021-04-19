#!/usr/bin/env bash

os=$(uname)
arch=$(uname -m)
filename="renote-linux-amd64"

case $os in
"Linux")
  case $arch in
  "x86_64")
    filename="renote-linux-amd64"
    ;;
  *)
    echo "The architecture ($arch) is not supported" >/dev/stderr
    exit 1
    ;;
  esac
  ;;
"Darwin")
  filename="renote-darwin-amd64"
  ;;
*)
  echo "The platform ($os) is not supported" >/dev/stderr
  exit 1
  ;;
esac

echo $filename
