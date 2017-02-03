size_t rio_unget(rio_t* r,c){
    r->rio_cnt++;
    r->rio_bufptr--;
}

char peek(const rio_t* r){ return *r->rio_bufptr; }

size_t rio_ungetc(rio_t*,c){r->rio_cnt++; r->rio_bufptr--;*r->rio_bufptr=c }

int main(){
    long *a;
    int b = a;
    printf("%d",++a - b);
}
