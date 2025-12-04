int func() {
    if (1 == 2) {
        return 0;
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
