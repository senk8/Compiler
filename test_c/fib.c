int fib(int num){
    if(num == 0){
       return 0;
    }
    if(num == 1){
        return 1;
    }
    return fib(num - 1) + fib(num - 2);
}

int main(){
    int x;
    int y;
    x = 5;
    y = 12 ;
    return fib(5);
}
