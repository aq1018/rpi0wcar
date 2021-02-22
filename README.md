# rpi0wcar
Bluetooth game pad controlled 3D printed RC Car with Raspberry Pi Zero W

## Hardware

I used [`openzcar`](https://github.com/alexyu132/zcar) as the basis of this project, and modified the electronics. Please follow the instruction there to build your RC car. The main reason I chose his design is due to its simplicity and I had most of the parts needed on hand. You can choose any RC car design as long as it uses a brushed motor for throttle, and a servo for steering.

For electronics, I used:

1. Raspberry PI Zero W
2. [Motor Driver Hat for Raspberry Pi ( Amazon )](https://www.amazon.com/gp/product/B07K7NP7C9/ref=ppx_yo_dt_b_asin_title_o06_s00?ie=UTF8&psc=1)
3. [JST Connectors (optional, but safer)](https://www.amazon.com/gp/product/B07R4ZBBC4/ref=ppx_yo_dt_b_asin_title_o05_s00?ie=UTF8&psc=1)

I chose to use the Motor Driver Hat because it also doubles as a Linear Voltage Regulator to power the Pi. I initially used a buck converter and hand made L293D circuit board, and it was a bit bigger and much messier than the Motor Driver Hat, so I changed my design to use the Hat instead.

## Setup Your Raspberry Pi Zero W

### Preparing your PI

follow this guide to setup your Pi Zero with headless ssh access: https://desertbot.io/blog/headless-raspberry-pi-3-bplus-ssh-wifi-setup


### Update System

```bash
sudo apt update -y
sudo apt dist-upgrade -y
sudo reboot
```

### Enable I2C

Login via SSH, and type the following:

```bash
sudo raspi-config
```

Navigate to: `Interface Options` -> `I2C`, and select Yes, then save and exit.

### Enable Hardware PWM

```bash
echo 'dtoverlay=pwm' | sudo tee -a /boot/config.txt`
sudo reboot
```

### Connect PS4 controller via Bluetooth

Run `bluetoothctl` command to open the bluetoothctl REPL. Under the prompt type the following to start scanning for bluetooth devices:

```
default-agent
power on
scan on
```

Hold the `Share` and `PS` button on the controller until controller light starts to flash. Your PS4 controller is now discoverable. `bluetoothctl` REPL should report pick up your controller and print its information. Find the MAC address of your PS 4 controller in the bluetoothctl REPL. It should look similar to blew:

```
[NEW] Device AA:BB:CC:DD:EE:FF Wireless Controller
```

Copy the MAC address (`AA:BB:CC:DD:EE:FF` in the example above ) and type the following to your REPL terminal:

```
pair AA:BB:CC:DD:EE:FF
```

Wait a bit, and type `yes` when asked to authorize. Now type:

```
connect AA:BB:CC:DD:EE:FF
trust AA:BB:CC:DD:EE:FF
```

At this point the light on the PS4 Controller should turned blue, indiciating it is now connected to the Pi.

This completes the bluetooth pairing process, and you can type `exit` to quit the REPL prompt.

### Install `rpi0wcar.deb`

I made a `deb` package for easy installation. 

* Up on installation, the package creates a `systemd` service and enable/starts the app automatically.
* There is a config file located at `/usr/local/etc/rpi0wcar.json`. You can customize it to fit your projects needs. e.g., setting offset duty cycles for steering, as every servo is slightly different due to manufactoring differences. The default json file looks like this:

```json
{
    "physical": {
        "steer_max_angle": 45.0,
        "steer_min_angle": -45.0
    },
    "motor": {
        "channel": "A",
        "i2c_address": 64,
        "prescale": 100
    },
    "servo": {
        "offset_duty":-0.006,
        "pwm_channel":"Pwm0"
    }
}
```

To install the package:

```bash
cd /tmp
curl https://github.com/aq1018/rpi0wcar/releases/download/v0.1.0/rpi0wcar_0.1.0_armhf.deb -O
sudo dpkg -i /tmp/rpi0wcar_0.1.0_armhf.deb
rm /tmp/rpi0wcar_0.1.0_armhf.deb
```

Verify `rpi0wcar` is running:

```
ps aux | grep rpi0wcar
```

You should see the app is running, and you should be able to control your RC car via a PS4 controller.

- [TODO] Add photos.
- [TODO] Add modified bottom plate STL file ( enlarged it in order to secure rpi 0w).
- [TODO] Document `rpi0wcar.json` cnofig file.
