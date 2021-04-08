github:
	open https://github.com/samlet/bluecc

substrate:
    substrate --dev --tmp --unsafe-ws-external --rpc-cors=all --unsafe-rpc-external --rpc-methods=Unsafe --prometheus-external
canvas:
    canvas --name local_node --tmp -lruntime=debug --ws-port 9944 --dev

