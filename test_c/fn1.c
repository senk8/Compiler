int test(int a,int b,int c,int d,int e,int f){
    return a+b+c+d+e+f;
}

int main(){
    int x;
    int y;
    int z;
    int* a;
    x = test(1,2,3,4,5,6)+2;
    y = 12 ;
    a = &x;
    *a = 12;
    y = &y + 1;
    return *a;
}
