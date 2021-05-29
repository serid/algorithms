#pragma once

#include <stdlib.h>

#include "ints.h"

typedef struct {
    size_t size;
    void *data;
} vec;

#define vecNew() \
    ((vec) { .size = 0, .data = NULL })

// Returns vec
#define vecNewWithSize(size_arg, T) \
    ({ \
        size_t VecResize_size = size_arg; \
        (vec) { .size = VecResize_size, .data = malloc(VecResize_size * sizeof(T)) }; \
    })

#define vecPush(self, value, T) \
    do { \
        vec *VecPush_selfP = &(self); \
        vecResize(*VecPush_selfP, VecPush_selfP->size + 1, T); \
        ((T*)(VecPush_selfP->data))[VecPush_selfP->size - 1] = (value); \
    } while(0)

// Returns T
#define vecPop(self, T) \
    ({ \
        vec *VecPop_selfP = &(self); \
        T VecPop_result = ((T*)(VecPop_selfP->data))[VecPop_selfP->size - 1]; \
        vecResize(*VecPop_selfP, VecPop_selfP->size - 1, T); \
        VecPop_result; \
    })

#define vecResize(self, size_arg, T) \
    do { \
        vec *VecResize_selfP = &(self); \
        size_t VecResize_size = (size_arg); \
        if (VecResize_size == 0) { \
            free(VecResize_selfP->data); \
            VecResize_selfP->data = NULL; \
        } else { \
            VecResize_selfP->data = realloc(VecResize_selfP->data, VecResize_size * sizeof(T)); \
        } \
        VecResize_selfP->size = VecResize_size; \
    } while(0)

// Can be used for both getting and setting values
#define vecGetUnchecked(self, index, T) \
    (((T*)(self).data)[index])

#define vecGetPointer(self, index, T) \
    ({ \
        vec *VecGet_selfP = &(self); \
        size_t VecGet_index = index; \
        if (VecGet_index > VecGet_selfP->size) { \
            fprintf(stderr, "index out of bounds @ " __FILE__ ": %i", __LINE__); \
            exit(1); \
        } \
        &vecGetUnchecked(*VecGet_selfP, VecGet_index, T); \
    })

#define vecGet(self, index, T) \
    (*vecGetPointer(self, index, T))

#define vecRemove(self, index, T) \
    do { \
        vec *VecRemove_selfP = &(self); \
        for (int VecRemove_i = index; VecRemove_i < VecRemove_selfP->size - 1; VecRemove_i++) { \
            vecGetUnchecked(*VecRemove_selfP, VecRemove_i, T) = vecGetUnchecked(*VecRemove_selfP, VecRemove_i + 1, T); \
        }                                    \
        vecResize(*VecRemove_selfP, VecRemove_selfP->size - 1, T); \
    } while(0)

#define vecClone(self, T) \
    ({ \
        vec *VecClone_selfP = &(self); \
        (vec) {.size = VecClone_selfP->size, .data = malloc_memcpy(VecClone_selfP->data, VecClone_selfP->size * sizeof(T))}; \
    })

#define vecDestroy(self) \
    do { \
        vec *VecDestroy_selfP = &(self); \
        free(VecDestroy_selfP->data); \
        VecDestroy_selfP->data = NULL; \
        VecDestroy_selfP->size = 0; \
    } while(0)
    