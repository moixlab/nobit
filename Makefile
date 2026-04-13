.PHONY: help build clean test assets

help:
	@echo "make [options] [target]\nTargets:\n \
	help  - Lista de targets.\n \
	build - Compila código.\n \
	clean - Limpia ejecutables.\n \
	test  - Ejecuta pruebas.\n \
	pkgs  - Instala paquetes.\n"

build:
	@echo "Compilando...\n"
	mkdir -p ./bin/
	cargo build --release -p nobit
	cp -f ./target/release/nobit ./bin/

clean:
	@echo "Limpiando ejecutables...\n"
	rm -rf ./bin/

test:
	@echo "Ejecutando pruebas...\n"

pkgs:
	@echo "Instalando paquetes...\n"
	sudo rm -f /etc/apt/sources.list.d/yarn.*
	sudo apt update -y
	sudo apt install -y \
		rustup
	rustup default stable