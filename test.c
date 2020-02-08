#include <stdio.h>
#include "mylib/mylib.h"

int tests_run = 0;

// https://eradman.com/posts/tdd-in-c.html
#define FAIL(expr, val) printf("\nfailure in %s() line %d: %s != %s\n", __func__, __LINE__, #expr, #val)
#define _assert(expr, val) do { if ((expr) != (val)) { FAIL(expr, val); return 1; } } while(0)
#define _verify(test) do { int r=test(); tests_run++; if(r) return r; } while(0)

int add_positive_numbers() {
    _assert(add(1,2), 3);
    return 0;
}

int add_negative_numbers() {
    _assert(add(-1,-2), -3);
    return 0;
}

int all_tests() {
    _verify(add_positive_numbers);
    _verify(add_negative_numbers);
    return 0;
}

int main(int argc, char **argv) {
    int result = all_tests();
    if (result == 0)
        printf("PASSED\n");
    printf("Tests run: %d\n", tests_run);

    return result != 0;
}

