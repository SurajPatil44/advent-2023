#include <stdio.h>

#define SV_IMPLEMENTATION
#include "../../includes/SV.h"

#define file_impl
#include "../../includes/file_reads.h"

int main(int argc, char** argv){
    char* filename = "../sample.txt";
    if(argc > 1) {
        filename = argv[1];
    }

    //printf("%s",
    BuffFile *res = init_buffered_file_io(filename);
    char buffer[512] = {0};
    while(1) {
        int sz = read_buffered_line(res,buffer);
        String_View sv = sv_from_cstr(buffer);
        if(sz == 0){
            break;
        };
        while(1) {
            String_View sv2 = sv_trim(sv_chop_by_delim(&sv,','));
            printf("SV1 : "SV_Fmt"\n",SV_Arg(sv));
            printf("SV2 : "SV_Fmt"\n",SV_Arg(sv2));
            if(sv.count == 0) break;
        }
        //printf("%s",buffer);
        //(void) sv;
        printf("\n===============================\n");
    }

    (void) res;
}
