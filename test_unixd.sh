#!/bin/bash

set -e

cd cpcemu_parser && \
	cargo build && cd ..

gcc \
	fake_emulator/main.c \
	-I./cpcemu_parser -I. \
	-lcpcemu_parser -Lcpcemu_parser/target/debug/ \
	-g \
	-o fkemud
	

./fkemud "error" || true
./fkemud "mem read 0x500"
./fkemud "mem read label"
./fkemud "mem write here that"
./fkemud "mem write 1234 26"
./fkemud "brk add toto"
./fkemud "brk add 58"