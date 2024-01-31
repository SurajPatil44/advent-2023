#ifndef file_reads_H_
#define file_reads_H_

#include<stdlib.h>
#include<stdio.h>
#include<stdbool.h>

#ifndef FDEF
#define FDEF
#endif

typedef struct BuffFile {
    FILE *f;
    size_t current_cursor;
    bool eof;
} BuffFile;

FDEF BuffFile* init_buffered_file_io(char* fname);
FDEF int read_buffered_line(BuffFile*,char*);


#endif //file_reads_H_
#ifdef file_impl

FDEF BuffFile* init_buffered_file_io(char* fname){
    BuffFile* B = (BuffFile*) malloc(sizeof(BuffFile));
    if(B == NULL) {
        // set errno
        return NULL;
    }
    FILE* f = fopen(fname,"r+");
    if(f==NULL) {
        free(B);
        //set errno
        return NULL;
    }
    B->f = f;
    B->current_cursor = 0;
    fseek(B->f, 0L, SEEK_END);
    size_t sz = ftell(B->f);
    if(sz > 0) {
        B->eof = false;
        rewind(B->f);
    } else {
        //empty file
        B->eof = true;
    }
    return B;
}

FDEF int read_buffered_line(BuffFile* bf,char* buffer) {
    if(bf->eof) return 0;
    size_t bufsize = 512;
    size_t characters = getline(&buffer, &bufsize, bf->f);
    //printf("characters %zu \n",characters);
    if((int) characters == -1) {
        bf->eof = true;
        return 0;
    }
    return characters;
}
#endif
