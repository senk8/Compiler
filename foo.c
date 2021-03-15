int fib(int num){
    if(num == 0){
       return 0;
    }
    if(num == 1){
        return 1;
    }
    return fib(num - 1) + fib(num - 2);
}

int test(int a,int b,int c,int d,int e,int f){
    return a+b+c+d+e+f;
}

int hoge(int a,int* b,int** c){
    return 12;
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
    return *a;
}
