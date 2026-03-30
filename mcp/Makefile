.PHONY: stop build restart

stop:
	@echo "Arret du serveur MCP (mcp.exe)..."
	@powershell -Command "Stop-Process -Name mcp -Force -ErrorAction SilentlyContinue" || exit 0

build: stop
	@echo "Compilation du projet Rust..."
	cargo build

restart: build
	@echo "Compilation terminee."
	@echo "Note : Le redemarrage se fait automatiquement par l'extension VS Code (ou Claude) des que vous lui posez une nouvelle question."