#! /bin/bash
# shellcheck disable=SC2035
cd ./flatten || exit $?
sha256sum * >../sha256sum.txt || exit $?