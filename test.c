#include <stdio.h>
#include <stdlib.h>

int main(){
    char* c = "+4";
    c++;
    printf(strtol(c,&c,10));
    return 0;
}
