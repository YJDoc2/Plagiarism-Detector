all : build


build:
	cd ./lexer && flex -ts lexer.l>lexer.c && gcc -c -g lexer.c && ar -r liblexer.a lexer.o
	cargo build