#!/usr/bin/env bash
sleep 1
echo "=== /health ==="
curl -sS http://127.0.0.1:3000/health || true
echo
echo "=== GET /items ==="
curl -sS http://127.0.0.1:3000/items || true
echo
echo "=== POST /items ==="
curl -sS -H "Content-Type: application/json" -d '{"id":2,"name":"New Item"}' http://127.0.0.1:3000/items || true
echo
echo "=== done ==="
