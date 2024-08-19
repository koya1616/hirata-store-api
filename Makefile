up:
	docker compose -f compose.dev.yaml up --build

exec:
	docker exec -it server /bin/bash

prod-up:
	docker compose -f compose.yaml up --build
