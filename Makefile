build:
	docker build -t ramlich .

run:
	docker run -d --name ramlich-run -p 8181:8181 ramlich:latest

stop:
	docker stop ramlich-run
	docker rm ramlich-run

rmi:
	docker image rm ramlich
