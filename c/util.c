#include "util.h"

#include <memory.h>
#include <string.h>

void *malloc_memcpy(void *p, size_t size) {
    return memcpy(malloc(size), p, size);
}

char *clone_str(const char *str) {
    return strcpy(malloc(strlen(str) + 1), str);
}