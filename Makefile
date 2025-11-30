# Dev Toolbox Makefile

.PHONY: help up down ps restart \
    py-shell ps-shell go-shell dotnet-shell rust-shell ansible-shell \
    shell-% logs logs-% \
    ansible-bootstrap rust-build rust-run rust-check py-health \
    stop-dev rm-dev nuke clean rebuild health \
    helath heaIth healt heath halth healht

help:
	@echo "Dev Toolbox Makefile"
	@echo ""
	@echo "  make up          - start all dev containers"
	@echo "  make rust-build  - build homelab-helper (Rust)"
	@echo "  make rust-check  - run full health check"
	@echo "  make helath      - typo alias for health"
	@echo ""

up:
	docker compose up -d

down:
	docker compose down

ps:
	docker ps

restart:
	docker compose down
	docker compose up -d

doctor:
	@echo "=== [1/3] Rust full check (homelab-helper) ==="
	docker exec -it dev-rust bash -lc "source /root/.cargo/env && cd /workspace/homelab-helper && ./target/release/homelab-helper check 192.168.2.51 || exit 1"
	@echo ""
	@echo "=== [2/3] Python HTTP suite checks ==="
	docker exec -it dev-python python /workspace/healthchecks/health_runner.py /workspace/healthchecks/checks.yaml || exit 1
	@echo ""
	@echo "=== [3/3] Ansible host ping (SSH check) ==="
	docker exec -it dev-ansible ansible -i /workspace/inventory/hosts.ini all -m ping
	@echo ""
	@echo "=== Lab doctor finished ==="

# -------------------------
# Rust targets
# -------------------------

rust-build:
	docker exec -it dev-rust bash -lc "source /root/.cargo/env && cd /workspace/homelab-helper && cargo build --release"

rust-run:
	docker exec -it dev-rust bash -lc "source /root/.cargo/env && cd /workspace/homelab-helper && ./target/release/homelab-helper ping 192.168.2.51"

rust-check:
	docker exec -it dev-rust bash -lc "source /root/.cargo/env && cd /workspace/homelab-helper && ./target/release/homelab-helper check 192.168.2.51"

# -------------------------
# Python health
# -------------------------

py-health:
	docker exec -it dev-python python /workspace/healthchecks/http_health_check.py http://192.168.2.51

py-suite:
	docker exec -it dev-python python /workspace/healthchecks/health_runner.py /workspace/healthchecks/checks.yaml

# -------------------------
# Maintenance commands
# -------------------------

stop-dev:
	-docker stop $$(docker ps -q -f "name=dev-") 2>/dev/null || true

rm-dev:
	-docker rm $$(docker ps -aq -f "name=dev-") 2>/dev/null || true

clean:
	-docker rm $$(docker ps -aq -f "name=dev-" -f "status=exited") 2>/dev/null || true

nuke:
	docker compose down --remove-orphans --volumes || true
	-docker rm -f $$(docker ps -aq) || true

rebuild: nuke
	docker compose up -d --build

# -------------------------
# Health bundle
# -------------------------

health: rust-check py-health
	@echo "=== Combined health checks complete ==="

# Typo aliases
helath: health
heaIth: health
heath: health
healt: health
halth: health
healht: health
