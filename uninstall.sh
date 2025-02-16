#!/usr/bin/env bash

[ "$EUID" -ne 0 ] && { echo 'This script must be run as root' >&2 ; exit 1 ;}

confirm() {
  local default="${2:-n}"
  local prompt="${1:-Are you sure?}"

  default="${default,,}"

  if [[ "$default" == "y" ]]; then
    prompt+=" [Y/n] "
  else
    prompt+=" [y/N] "
  fi

  while true; do
    read -r -p "$prompt" ans
    ans=${ans,,}

    [[ -z "$ans" ]] && ans="$default"

    case "$ans" in
      y|yes) return 0 ;;
      n|no) return 1 ;;
      *) echo "Please answer yes or no" >&2 ;;
    esac
  done
}

confirm "Uninstall 2?" || { echo 'Cancelled uninstall' ; exit 0 ;}
rm -rf /usr/share/2 \
       /usr/bin/2   \
       /etc/2
echo "Uninstalled 2"

confirm "Would you like to also remove '/usr/ports'?" || { echo 'Done' ; exit 0 ;}
rm -rf /usr/ports
echo "Removed /usr/ports"
