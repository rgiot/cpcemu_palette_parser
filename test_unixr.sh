#!/bin/bash

set -e

cd cpcemu_parser && \
	cargo build --release && cd ..

gcc \
	fake_emulator/main.c \
	-I./cpcemu_parser -I.\
	-lcpcemu_parser -Lcpcemu_parser/target/release/ \
	-O2 \
	-o fkemur
strip fkemur

./fkemur "error" || true
./fkemur "mem read 0x500"
./fkemur "mem read label"
./fkemur "mem write here that"
./fkemur "mem write 1234 26"
./fkemur "brk add toto"
./fkemur "brk add 58"