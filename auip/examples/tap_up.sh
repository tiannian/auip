ip tuntap add name tap0 mode tap
ip link set tap0 up
ip addr add 192.168.69.100/24 dev tap0