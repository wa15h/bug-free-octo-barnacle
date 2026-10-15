#!/usr/bin/env bash
# Points this clone's git at tools/hooks (W0-16), so every `git push` runs tools/hooks/pre-push first. Run it once
# per clone, from any directory; running it again changes nothing. Undo: git config --unset core.hooksPath.
set -euo pipefail
cd "$(dirname "$0")"
git config core.hooksPath tools/hooks
echo "install.sh: core.hooksPath is $(git config core.hooksPath); git push now runs tools/hooks/pre-push"
