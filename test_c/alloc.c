int main() {
    int* p;
    alloc(&p);
    p = p+1;
    return *p; 
}