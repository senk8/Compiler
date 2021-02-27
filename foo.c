fib(num){
    if(num == 0){
       return 0;
    }
    if(num == 1){
        return 1;
    }
    return fib(num - 1) + fib(num - 2);
}

test(a,b,c,d,e,f){
    return a+b+c+d+e+f;
}

main(){
    x = test(1,2,3,4,5,6);
    y = fib(2);
    z = fib(3);
    w = fib(4);
    return x+y+z+w;
}
