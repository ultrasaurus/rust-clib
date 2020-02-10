/* This is a very basic example header file */

#ifndef mylib_h
#define mylib_h

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdint.h>

typedef struct TcpConnection TcpConnection;

typedef struct my_thing_t my_thing_t;

typedef void (*my_status_callback_t)(int32_t code, void *user_data);

typedef void (*TcpStatusCallback)(int32_t code, void *user_data);

int32_t add(int32_t a, int32_t b);

my_thing_t *create_thing(int32_t num, my_status_callback_t callback_or_null, void *userdata);

int32_t thing_num(my_thing_t *thing_ptr);

void thing_something(my_thing_t *thing_ptr);

void destroy_thing(my_thing_t *thing_ptr);

TcpConnection *create_tcp(TcpStatusCallback callback, void *user_data);

void tcp_connect_blocking(TcpConnection *tcp_ptr);

void destroy_tcp(TcpConnection *tcp_ptr);

#endif /* mylib_h */
