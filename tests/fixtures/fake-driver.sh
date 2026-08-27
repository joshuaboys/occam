#!/bin/sh
# Test double for codex/claude/grok. Behaviour is controlled via env vars.
echo "diag: fake-driver" >&2

if [ -n "${OCCAM_FAKE_STDERR:-}" ]; then
  printf '%s' "$OCCAM_FAKE_STDERR" >&2
fi

if [ "${1:-}" = "--help" ]; then
  cat <<'EOF'
Usage: fake-driver
  exec <prompt>
  -p, --prompt <text>
EOF
  exit 0
fi

if [ -n "${OCCAM_FAKE_LOG:-}" ]; then
  {
    printf 'ARGV:'
    printf ' %s' "$0" "$@"
    printf '\n'
    if [ -t 0 ]; then
      echo 'STDIN:TTY'
    else
      echo "STDIN:$(cat)"
    fi
  } >> "$OCCAM_FAKE_LOG"
fi

if [ -n "${OCCAM_FAKE_SLEEP:-}" ]; then
  sleep "$OCCAM_FAKE_SLEEP"
fi

if [ -n "${OCCAM_FAKE_DESCENDANT_STDERR:-}" ]; then
  (
    sleep "${OCCAM_FAKE_DESCENDANT_DELAY:-0.1}"
    printf '%s' "$OCCAM_FAKE_DESCENDANT_STDERR" >&2
    sleep "${OCCAM_FAKE_DESCENDANT_SLEEP:-2}"
  ) &
  exit 0
fi

if [ -n "${OCCAM_FAKE_AUTH:-}" ]; then
  echo "not logged in: please log in" >&2
  exit 1
fi

if [ -n "${OCCAM_FAKE_COUNTER:-}" ]; then
  n=0
  if [ -f "$OCCAM_FAKE_COUNTER" ]; then
    n=$(cat "$OCCAM_FAKE_COUNTER")
  fi
  n=$((n + 1))
  echo "$n" > "$OCCAM_FAKE_COUNTER"
  if [ "$n" -eq 1 ] && [ -n "${OCCAM_FAKE_STDOUT_FIRST:-}" ]; then
    printf '%s' "$OCCAM_FAKE_STDOUT_FIRST"
    exit "${OCCAM_FAKE_EXIT:-0}"
  fi
fi

if [ -n "${OCCAM_FAKE_STDOUT_FILE:-}" ]; then
  cat "$OCCAM_FAKE_STDOUT_FILE"
elif [ -n "${OCCAM_FAKE_STDOUT:-}" ]; then
  printf '%s' "$OCCAM_FAKE_STDOUT"
else
  printf 'ok'
fi
exit "${OCCAM_FAKE_EXIT:-0}"
