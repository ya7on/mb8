int func(int a, int b) {
    int y = func(1,2);
    if (1 == 2) {
        return 9;
    }
    return 1;
}

int main() {
    int a = 1;
    if (a) {
        return 1;
    } else {
        return 0;
    }
}
