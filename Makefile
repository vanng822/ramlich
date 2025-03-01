build:
	make -j build-apiserver build-consumer

build-apiserver:
	docker build -t ramlich .

build-consumer:
	docker build -t ramlich_consumer -f consumer.Dockerfile .

run:
	docker run -d --name ramlich-run -p 8181:8181 ramlich:latest

run-consumer:
	docker run -d --name ramlich-consumer-run -p 8585:8585 ramlich_consumer:latest

stop:
	docker stop ramlich-run
	docker rm ramlich-run

stop-consumer:
	docker stop ramlich-consumer-run
	docker rm ramlich-consumer-run

rmi:
	docker image rm ramlich
