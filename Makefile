.PHONY: dev build spacetime-start spacetime-publish

dev:
	@echo "Starting devtools..."
	cd devtools && bun run dev &
	@echo "Starting game..."
	cd game && cargo tauri dev

build:
	cd devtools && bun run build
	cd game && cargo tauri build

spacetime-start:
	docker compose up -d

spacetime-publish:
	docker compose exec spacetimedb spacetime publish saros-db --project-path /app/module
