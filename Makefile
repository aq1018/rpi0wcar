bootstrap:
	docker build -t rpi0wcar/cross:latest .

build: bootstrap
	cross build --target=arm-unknown-linux-gnueabihf

clean:
	cross clean

deploy: build
	scp target/arm-unknown-linux-gnueabihf/debug/rpi0wcar pi@pi0wcar:

run: deploy
	ssh pi@pi0w ./rpi0wcar