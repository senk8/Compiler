#include <stdlib.h>

void alloc(int** p) {
    *p=(int*)malloc(sizeof(int)*4);
    (*p)[0] = 1;
    (*p)[1] = 2;
}

