# rpi0wcar
Bluetooth game pad controlled RC Car with Raspberry Pi Zero W

## Setup

1. follow this guide to setup your Pi Zero with headless ssh access: https://desertbot.io/blog/headless-raspberry-pi-3-bplus-ssh-wifi-setup
2. Update system: `sudo apt update -y&& sudo apt dist-upgrade -y`
3. Add permissions to access bluetooth and GPIO pins for `pi` user:
  1. `sudo usermod -aG bluetooth pi`
  2. `sudo usermod -aG gpio pi`
4. reboot system `sudo reboot`.
5. reconnect via ssh ( if you changed your hostname, you need to connect with the new hostname, eg. `ssh pi@<your new pi host name here>` ).
6. Enable I2c:  
  1. `sudo raspi-config`
  2. `Interface Options` -> `I2C` -> `Yes`
7. Enable Hardware PWM: ` echo 'dtoverlay=pwm' | sudo tee -a /boot/config.txt`
8. Connect your PS 4 controller via bluetooth:
  1. Run `bluetoothctl`
  2. Under the prompt type `default-agent`, you should see something like `Default agent request successful`
  3. `power on`
  4. `scan on`
  5. Hold share and PS button until controller light starts to flash.
  6. Find the MAC address of the PS 4 controller. My looked like this: `[NEW] Device 90:89:5F:19:66:13 Wireless Controller`, and my controller's MAC address is `90:89:5F:19:66:13`
  7. `pair <your-controller-mac-address>`, type `yes` when asked to authorize.
  8. `connect <your-controller-mac-address>`
  9. `trust <your-controller-mac-address>`
  10. `exit`
