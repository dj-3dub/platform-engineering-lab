#!/usr/bin/env python3
import sys
import ssl
import urllib.request
import urllib.error
import time
from typing import List, Dict, Any

import yaml


def load_checks(path: str) -> List[Dict[str, Any]]:
  with open(path, "r", encoding="utf-8") as f:
      data = yaml.safe_load(f)
  return data.get("checks", [])


def run_http_check(name: str, url: str, timeout: float, expect_status: List[int], verify_tls: bool) -> Dict[str, Any]:
  start = time.time()
  ctx = None

  if url.lower().startswith("https") and not verify_tls:
      ctx = ssl._create_unverified_context()

  try:
      req = urllib.request.Request(url, method="GET")
      with urllib.request.urlopen(req, timeout=timeout, context=ctx) as resp:
          status = resp.getcode()
          elapsed = (start - time.time()) * -1000.0
          result = {
              "name": name,
              "url": url,
              "status": status,
              "elapsed_ms": round(elapsed, 1),
          }
          if status in expect_status:
              result["ok"] = True
              result["message"] = f"OK (status={status}, {result['elapsed_ms']} ms)"
          else:
              result["ok"] = False
              result["message"] = f"FAIL (status={status}, {result['elapsed_ms']} ms, expected one of {expect_status})"
          return result
  except urllib.error.URLError as e:
      return {
          "name": name,
          "url": url,
          "ok": False,
          "status": None,
          "elapsed_ms": None,
          "message": f"ERROR: {e}"
      }
  except Exception as e:
      return {
          "name": name,
          "url": url,
          "ok": False,
          "status": None,
          "elapsed_ms": None,
          "message": f"ERROR: {e}"
      }


def main():
  if len(sys.argv) > 1:
      config_path = sys.argv[1]
  else:
      config_path = "/workspace/healthchecks/checks.yaml"

  print(f"Using config: {config_path}")
  checks = load_checks(config_path)
  if not checks:
      print("No checks defined in YAML.")
      sys.exit(2)

  results = []
  for item in checks:
      name = item.get("name", "unnamed-check")
      url = item["url"]
      timeout = float(item.get("timeout", 3))
      expect_status = item.get("expect_status", [200])
      verify_tls = bool(item.get("verify_tls", True))

      print(f"\n=== Running check: {name} ===")
      print(f"URL: {url}")
      res = run_http_check(name, url, timeout, expect_status, verify_tls)
      print(res["message"])
      results.append(res)

  # Summary
  total = len(results)
  ok_count = sum(1 for r in results if r.get("ok"))
  fail_count = total - ok_count

  print("\n=== HTTP Health Summary ===")
  print(f"Total checks: {total}")
  print(f"OK:          {ok_count}")
  print(f"Failed:      {fail_count}")

  # Exit code: 0 if all OK, 1 otherwise
  sys.exit(0 if fail_count == 0 else 1)


if __name__ == "__main__":
  main()
