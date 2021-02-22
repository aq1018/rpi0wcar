bootstrap:
	docker build -t rpi0wcar/cross:latest .

build: bootstrap
	cross build --target=arm-unknown-linux-gnueabihf --release
	cargo strip --target arm-unknown-linux-gnueabihf
	cargo-deb -v --no-build --target arm-unknown-linux-gnueabihf --no-strip

clean:
	cross clean

deploy: build
	scp target/arm-unknown-linux-gnueabihf/debian/rpi0wcar_0.1.0_armhf.deb pi@rpi0wcar:

run: deploy
	ssh pi@rpi0wcar ./rpi0wcar