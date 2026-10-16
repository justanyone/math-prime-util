#!/bin/sh
# Source this file from anywhere inside the checkout: `. tools/perl-env.sh`.
# Puts the project-local Perl library (.perl-local/, holds Math::Prime::Util 0.75,
# the fixture oracle) on PATH/PERL5LIB for the current shell.
# Bootstrap (no sudo):
#   perl <(curl -sL https://cpanmin.us) -l .perl-local --notest App::cpanminus local::lib
#   .perl-local/bin/cpanm -l .perl-local --notest Math::BigInt bignum Test::More Math::Prime::Util@0.75
_root="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
eval "$(perl -I"$_root/.perl-local/lib/perl5" -Mlocal::lib="$_root/.perl-local")"
unset _root
