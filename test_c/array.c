int main(){
    int a[2];
    *a = 1;
    int* b;
    b = a+1;
    *b = 2;
    int *p;
    p = a;
    return *p + *(p + 1) 
}