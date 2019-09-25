.PHONY: build

IMAGE:=doggo/doggo-api

build: 
	docker build -t $(IMAGE):latest -f build/Dockerfile .

clean:
	@echo 'stopping docker containers'
	@docker stop `docker ps -aq`
	@echo 'removing all doggo containers'
	@docker ps -a | awk '{ print $$1,$$2 }' | grep doggo | awk '{ print $$1 }' | xargs -I {} docker rm {}
