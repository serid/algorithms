// Garbage collector for C.
// It's of "precise" (non-conservative) and "non-moving" type so in theory it's faster then Boehm GC, but not as fast as it can get.
// To use the garbage collector the user has to provide two integral parts:

// I: Visitor for every type that will be used it a garbage collected heap
// Visitor functions will be called on every reachable object and they have to mark the object as live and transitively call visitors for child objects

// II: Root annotations for local and global variables
// If you don't want an object to be freed, you should either mark it as a root, or you should ensure that it will be transitively reachable from roots through visitors
// You don't have to mark every allocation as a root, in fact you shouldn't, since it will prevent garbage from being reclaimed
// You only need to root local and global variables and the GC will work it's way from there using visitors

#pragma once

#include <stdlib.h>
#include <stdbool.h>
#include <stdalign.h>
#include <stddef.h>

// Objects GC operates on
typedef void *ObjectP;

// virtual function called when the object is visited by GC to visit referenced objects
typedef void (*visitor_f)(void *);

// GC root
typedef struct {
    ObjectP object;
    visitor_f visitor;
} Root;

Root make_root(ObjectP object, visitor_f visitor);

// Macro makes a root from a variable. Object Visitor is infered using a provided TYPE_TO_VISITOR macro.
#define MAKE_ROOT(variable) \
    (make_root((variable), TYPE_TO_VISITOR(variable)))

// Example of what TYPE_TO_VISITOR macro may look like. You have to provide the macro and visitors yourself.
// Select object visitor based on variable type
//#define TYPE_TO_VISITOR(T) _Generic((T), \
//Expression*: visit_expression \
//MyObject*: visit_my_object \
//char*: noop_visitor \
//int*: noop_visitor
//)

#define GC_PUSH_ROOT(variable) \
    gc_push_root(MAKE_ROOT(variable))

#define GC_POP_ROOT(variable) \
    gc_pop_root(MAKE_ROOT(variable))

// Call at program startup
void gc_init(size_t heap_limit_);

// Call at program exit
void gc_deinit();

// It is possible to re-initialize GC after it has been deinitialized
// Any of the following gc_* functions should be called between gc_init() and gc_deinit()

// Allocate an object in a gc heap
// When NDEBUG is not defined exits with an error message on gc-out-of-memory or malloc-failure errors
ObjectP gc_alloc(size_t size);

// Returns NULL on out gc-heap-out-of-memory OR malloc-failure and sets the appropriate flag
ObjectP gc_alloc_full(size_t size, bool *is_gc_oom, bool *is_malloc_fail);

// Force immediate garbage collection
void gc_cleanup();

// Get and Set GC heap size limit
size_t gc_get_limit();
void gc_set_limit(size_t limit);

// Get number of bytes allocated for all objects in a GC heap
size_t gc_get_heap_size();

void gc_push_root(Root root);

Root gc_pop_root_unchecked();

void gc_pop_root(Root root);

// Useful for popping multiple roots without checking their order
// Variadic arguments should be of type Root
void gc_pop_roots(size_t count, ...);

// Function to be called by the visitors to mark object as live
void gc_mark_object(ObjectP object);