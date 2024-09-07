build:
	docker build -t ramlich .

build-consumer:
	docker build -t ramlich_consumer -f consumer.Dockerfile .

run:
	docker run --env RUST_BROKERS=host.docker.internal:29092 -d --name ramlich-run -p 8181:8181 ramlich:latest

run-consumer:
	docker run --env RUST_BROKERS=host.docker.internal:29092 -d --name ramlich-consumer-run ramlich_consumer:latest

stop:
	docker stop ramlich-run
	docker rm ramlich-run

stop-consumer:
	docker stop ramlich-consumer-run
	docker rm ramlich-consumer-run

rmi:
	docker image rm ramlich
