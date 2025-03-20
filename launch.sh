#!/usr/bin/env bash

if command -v sudo >/dev/null 2>&1; then
    S="sudo"
elif command -v doas >/dev/null 2>&1; then
    S="doas"
else
    S=""
fi

if [[ -n "${S}" ]]; then
    exec "${S}" env LOG_LEVEL="${LOG_LEVEL}" /usr/libexec/two "$@"
else
    exec env LOG_LEVEL="${LOG_LEVEL}" /usr/libexec/two "$@"
fi
