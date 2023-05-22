#source dashboard.gdb
tui enable
layout split
file ./target/thumbv7em-none-eabihf/debug/bitsyrs
target extended-remote /dev/ttyACM0
monitor connect_srst enable
monitor swdp_scan
attach 1
load
