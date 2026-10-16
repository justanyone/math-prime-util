#!/bin/sh
# Source this file from anywhere inside the checkout: `. tools/perl-env.sh`.
# Puts the project-local Perl library (.perl-local/, holds Math::Prime::Util 0.75,
# the fixture oracle) on PATH/PERL5LIB for the current shell, and bootstraps it
# (no sudo, needs network, a few minutes) when it is missing.
_root="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
if ! perl -I"$_root/.perl-local/lib/perl5" -Mlocal::lib="$_root/.perl-local" -MMath::Prime::Util -e1 2>/dev/null; then
  echo "perl-env: bootstrapping $_root/.perl-local (Math::Prime::Util 0.75)" >&2
  curl -sL https://cpanmin.us | nice -n 10 perl - -l "$_root/.perl-local" --notest App::cpanminus local::lib >/dev/null \
    && nice -n 10 "$_root/.perl-local/bin/cpanm" -l "$_root/.perl-local" --notest Math::BigInt bignum Test::More Math::Prime::Util@0.75 >/dev/null \
    || echo "perl-env: bootstrap failed" >&2
fi
eval "$(perl -I"$_root/.perl-local/lib/perl5" -Mlocal::lib="$_root/.perl-local")"
unset _root
