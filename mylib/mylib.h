/* This is a very basic example header file */

#ifndef mylib_h
#define mylib_h

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdint.h>

typedef struct my_thing_t my_thing_t;

int32_t add(int32_t a, int32_t b);

my_thing_t *create_thing(int32_t num);

void destroy_thing(my_thing_t *thing_ptr);

int32_t thing_num(my_thing_t *thing_ptr);

#endif /* mylib_h */
