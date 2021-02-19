bootstrap:
	docker build -t rpi0wcar/cross:latest .

build: bootstrap
	cross build --target=arm-unknown-linux-gnueabihf --release
	cargo strip --target armv-unknown-linux-gnueabihf

clean:
	cross clean

deploy: build
	scp target/arm-unknown-linux-gnueabihf/release/rpi0wcar pi@rpi0wcar:

run: deploy
	ssh pi@rpi0wcar ./rpi0wcar