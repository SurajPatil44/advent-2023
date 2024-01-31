#!/bin/sh

set -xe



CFLAGS="-Wall -Wextra -std=c11 -Werror -pedantic -ggdb -v"

##cc $CFLAGS -o ./objects/SV.o ./../includes/SV.h 
##cc $CFLAGS -o ./objects/file.o ./../includes/file_reads.h


SRC="./src/main.c"

DST="./build/main"

cc $CFLAGS -o $DST $SRC -lm -I./../includes

