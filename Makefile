TARGET=armv5te-unknown-linux-musleabi
ROUTER_USER=root
ROUTER_IP=192.168.1.1
ROUTER_DIRECTORY=/jffs
API_URL=
MACHINE_NAME=

RUST_FILES = $(shell find . -iname *.rs)

build-release:
	cross build --target ${TARGET} --release -p dd-wrt-wol-cli

copy-release: build-release
	@ssh ${ROUTER_USER}@${ROUTER_IP} killall dd-wrt-wol-cli || true
	scp target/${TARGET}/release/dd-wrt-wol-cli ${ROUTER_USER}@${ROUTER_IP}:${ROUTER_DIRECTORY}/

run: copy-release
	ssh ${ROUTER_USER}@${ROUTER_IP} RUST_LOG=trace ${ROUTER_DIRECTORY}/dd-wrt-wol-cli --machine-name=${MACHINE_NAME} -u ${API_URL}

build-api:
	cross build --target x86_64-unknown-linux-musl --release -p dd-wrt-wol-api

.PHONY=build-release copy-release run build-api

bin/dd-wrt-wol-api: $(RUST_FILES)
	mkdir -p bin
	cargo build --release
	cp target/release/dd-wrt-wol-api ./bin/

docker-push: bin/dd-wrt-wol-api
	docker build -f local.Dockerfile -t jaysonsantos/dd-wrt-wol:tracing .
	docker push jaysonsantos/dd-wrt-wol:tracing

.PHONY=docker-push
