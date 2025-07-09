# Makefile for managing Docker containers
start_db:
	docker-compose up -d

stop_db:
	docker-compose down

clean:
	docker-compose down --volumes --remove-orphans