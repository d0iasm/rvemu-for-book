int main() {
    volatile char *uart = (volatile char *) 0x10000000;
    uart[0] = 'H';
    uart[0] = 'e';
    uart[0] = 'l';
    uart[0] = 'l';
    uart[0] = 'o';
    uart[0] = ',';
    uart[0] = ' ';
    uart[0] = 'w';
    uart[0] = 'o';
    uart[0] = 'r';
    uart[0] = 'l';
    uart[0] = 'd';
    uart[0] = '!';
    uart[0] = '\n';
    return 0;
}

