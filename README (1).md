# ⚙️ Platform Engineering Lab  
### A multi-language DevOps toolbox for automation, health checking, and containerized development

<div align="center">

<img src="https://img.shields.io/badge/Rust-Healthcheck-orange?style=for-the-badge">
<img src="https://img.shields.io/badge/Python-Monitoring-blue?style=for-the-badge">
<img src="https://img.shields.io/badge/Ansible-Bootstrap-red?style=for-the-badge">
<img src="https://img.shields.io/badge/DevContainers-Docker-brightgreen?style=for-the-badge">

**Modern. Modular. Reproducible.**  
Everything you need to build, test, automate, and explore DevOps workflows inside a fully containerized environment.

</div>

---

## 📦 Overview

The **Platform Engineering Lab** is a full-stack engineering environment built around:

- ⚙️ **DevContainers** for isolated development  
- 🦀 **Rust** for fast health-checking CLI tools  
- 🐍 **Python** for HTTP monitoring + YAML-driven checks  
- 🤖 **Ansible** for host bootstrap & configuration  
- ☁️ **Terraform** + **AWS CLI** for cloud workflows  
- 🐳 **Docker Compose** for consistent, portable dev setups  
- 🧰 A unified **Makefile** for smooth automation  

This repo acts as your **internal developer platform**, fully contained in Docker and designed for Platform Engineering, DevOps, SRE, and modern homelab workflows.

---

## 🗂 Directory Structure

```
platform-engineering-lab/
├── ansible/                 # Bootstrap, config mgmt, roles
│   ├── ansible.cfg
│   ├── inventory/
│   ├── playbooks/
│   └── roles/
├── python/
│   └── healthchecks/        # YAML-driven HTTP checks
├── rust/
│   └── homelab-helper/      # Rust CLI health suite
├── terraform/               # IaC tooling container
├── docker-compose.yml       # Multi-container dev stack
├── Dockerfile               # Universal DevContainer
├── Makefile                 # Automation control center
└── .devcontainer/           # VS Code DevContainer config
```

---

## 🛠 Tech Stack

| Tech | Purpose |
|------|---------|
| **Rust** | Fast, parallelized health-check CLI (`homelab-helper`) |
| **Python** | HTTP monitoring + YAML-defined checks |
| **Ansible** | Host bootstrap & configuration roles |
| **Terraform** | Infrastructure-as-Code workflows |
| **AWS CLI** | AWS automation & credential handling |
| **DevContainers** | Portable multi-language development |
| **Docker Compose** | Container orchestration |
| **GH CLI** | GitHub automation (repos, auth, CI) |

---

## 🚀 Getting Started

### 1. Clone the repo:

```bash
git clone https://github.com/dj-3dub/platform-engineering-lab.git
cd platform-engineering-lab
```

### 2. Bring up the dev toolchain:

```bash
make up
```

This launches all language containers:

- dev-python  
- dev-rust  
- dev-ghcli  
- dev-ansible  
- dev-terraform  
- dev-dotnet  
- dev-go  
- dev-powershell  

### 3. Enter any container:

```bash
docker exec -it dev-python bash
docker exec -it dev-rust bash
docker exec -it dev-ghcli bash
```

---

## 🦀 Rust: `homelab-helper`

A fast, containerized CLI for host & service health checking.

### Build:

```bash
make rust-build
```

### Run a TCP scan:

```bash
make rust-check HOST=192.168.2.51
```

### Manual run:

```bash
docker exec -it dev-rust bash
cd /workspace/homelab-helper
cargo run -- scan 192.168.2.51 20 30
```

---

## 🐍 Python Healthchecks

Located at: `python/healthchecks/`

Driven by YAML:

**checks.yaml**

```yaml
services:
  - name: Nextcloud
    url: http://192.168.2.51:8085
  - name: Grafana
    url: http://192.168.2.60:3000
```

Run:

```bash
make python-check
```

Or manually:

```bash
docker exec -it dev-python bash
python3 health_runner.py checks.yaml
```

---

## 🤖 Ansible Bootstrap

Located at: `ansible/playbooks/bootstrap.yml`

Run against your inventory:

```bash
docker exec -it dev-ansible bash
ansible-playbook playbooks/bootstrap.yml -i inventory/hosts.ini
```

Roles include:

- ubuntu-base  
- rhel-base  
- devtools  
- docker-host  
- monitoring-agent  
- security-hardening  

---

## 🧰 Automation with Makefile

Key commands:

```bash
make up             # Start containers
make down           # Stop containers
make health         # Run Rust + Python checks
make rust-build     # Build Rust CLI
make python-check   # Run Python checks
make clean          # Cleanup everything
```

---

## 🧭 VS Code DevContainer Support

Open in VS Code → **Reopen in Container**

You instantly get:

- .NET 10 SDK  
- Terraform  
- AWS CLI  
- GH CLI  
- Unified workspace  
- Shell integration & preinstalled tools  

---

## 🌅 Roadmap

- 🔧 GitHub Actions CI/CD  
- 📊 Health dashboard (HTML or TUI)  
- 📡 Slack/Discord notifications  
- 🐍 Python enrichment (latency probing)  
- 🛡 Hardened Ansible roles  
- ☁️ Terraform modules  
- 🎛 `make doctor` all-in-one health summary  

---

## 📜 License

MIT License

---

<div align="center">

**Made for Platform Engineers.  
Built for automation.  
Designed for the future.**

</div>
