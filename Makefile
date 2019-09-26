.PHONY: build

IMAGE:=registry.planetscale.com/kubecon/goodestdoggo

build: 
	docker build -t $(IMAGE):demo -f build/Dockerfile .

clean:
	@echo 'stopping docker containers'
	@docker stop `docker ps -aq`
	@echo 'removing all doggo containers'
	@docker ps -a | awk '{ print $$1,$$2 }' | grep doggo | awk '{ print $$1 }' | xargs -I {} docker rm {}
