TCPCLIENTDIRS=tcp-client
TCPECHOSERVERDIRS=tcp-echo-server

.PHONY: tcpechoserver tcpclient

tcpechoserver:
	cd ${TCPECHOSERVERDIRS} && cargo run

tcpclient:
	cd ${TCPCLIENTDIRS} && cargo run
