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

int test_create_thing() {
  my_thing_t* thing = create_thing(1, NULL, NULL);
  int32_t num = thing_num(thing);
  _assert(num, 1);
  destroy_thing(thing);
  return 0;
}

typedef struct {
  int counter;
} TestThingStatusData;

void status_callback(int32_t code, void *user_data) {
  TestThingStatusData* my_data = (TestThingStatusData*)user_data;
  my_data->counter++;
}

int test_thing_callback() {
  TestThingStatusData data = {0}; 
  my_thing_t* thing = create_thing(1, status_callback, &data);
  thing_something(thing);
  _assert(data.counter, 1);
  destroy_thing(thing);
  return 0;
}

typedef struct {
  int counter;
} TestTcpStatusData;

void * tcp_status_callback(int32_t code, void *user_data) {
  TestTcpStatusData* my_data = (TestTcpStatusData*)user_data;
  my_data->counter++;
  return user_data;
}

int test_tcp_create_connect_blocking() {
  TestTcpStatusData data = {0};
  TcpConnection* tcp = create_tcp(status_callback, &data);
  tcp_connect_blocking(tcp);
  _assert(data.counter, 1);
  destroy_tcp(tcp);
  return 0;
}

int test_tcp_create_connect_async() {
  TestTcpStatusData data = {0};
  TcpConnection* tcp = create_tcp(status_callback, &data);
  tcp_connect_async(tcp);
  _assert(data.counter, 1);
  destroy_tcp(tcp);
  return 0;
}


// would be good to test memory leaks
int all_tests() {
    _verify(add_positive_numbers);
    _verify(add_negative_numbers);
    _verify(test_create_thing);
    _verify(test_thing_callback);
    _verify(test_tcp_create_connect_blocking);
    _verify(test_tcp_create_connect_async);
    return 0;
}

int main(int argc, char **argv) {
    int result = all_tests();
    if (result == 0)
        printf("PASSED\n");
    printf("Tests run: %d\n", tests_run);

    return result != 0;
}

