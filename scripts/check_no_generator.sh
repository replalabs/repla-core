#!/usr/bin/env bash
set -euo pipefail
# Run before every push: refuses if any generator file is tracked.
if git ls-files | grep -iE '^(main|generator|builder)\.py$' >/dev/null; then
  echo "FATAL: generator file is tracked. Untrack it before pushing." >&2
  git ls-files | grep -iE '^(main|generator|builder)\.py$' >&2
  exit 1
fi
# Refuse if any commit author hits Cryptottat.
if git log --format='%an <%ae>' | grep -i cryptottat >/dev/null; then
  echo "FATAL: Cryptottat identity detected in commit log." >&2
  exit 1
fi
echo "ok: no generator + no Cryptottat in log."
