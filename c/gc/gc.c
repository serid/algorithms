#include "gc.h"

#include <stdio.h>
#include <stdarg.h>

#include "../vec.h"

// GC header is placed before the object in block of memory returned by malloc
// Note that there is padding between GC header and the object, so block_size > sizeof(GcHeader) + sizeof(Object)
typedef struct {
    bool marked; // is the object marked by the GC
//    visitor_f f; // virtual function to be call when the object is visited by GC
} GcHeader;

typedef GcHeader *GcHeaderP;

// Allocated objects are assumed to have maximum alignment supported. This is what malloc does, too.
#define object_alignment alignof(max_align_t)

// object should be aligned by alignof(max_align_t) therefore size of a header is taken to be smallest number divisible by alignof(max_align_t) and higher than sizeof(GcHeader)
#define header_size_in_allocation (((sizeof(GcHeader) / object_alignment) + 1) * object_alignment)

// Private type to keep track of blocks allocated by GC
typedef struct {
    GcHeaderP header;
    size_t allocation_size;
} Allocation;

// Global GC state
// Maximum heap size
static size_t heap_limit;
// Number of bytes currently allocated
static size_t heap_size;
// vec of Allocation to object headers
static vec allocations;
// vec of Root structs
static vec roots;


Root make_root(ObjectP object, visitor_f visitor) {
    Root r = {
            .object = object,
            .visitor = visitor,
    };
    return r;
}

static Allocation make_allocation(GcHeaderP header, size_t allocation_size) {
    Allocation a = {
            .header = header,
            .allocation_size = allocation_size,
    };
    return a;
}

static GcHeaderP objectP_to_gc_headerP(ObjectP object) {
    return object - header_size_in_allocation;
}

static ObjectP gc_headerP_to_objectP(GcHeaderP header) {
    return header + header_size_in_allocation;
}

static void gc_mark() {
    for (int i = 0; i < roots.size; ++i) {
        Root r = vecGet(roots, i, Root);
        r.visitor(r.object);
    }
}

static void gc_sweep() {
    // Free unmarked objects and remove them from "allocations" vector
    for (int i = 0; i < allocations.size;) {
        Allocation allocation = vecGet(allocations, i, Allocation);
        if (allocation.header->marked) {
            allocation.header->marked = false;
            ++i;
        } else {
            free(allocation.header);
            heap_size -= allocation.allocation_size;
            vecRemove(allocations, i, Allocation);
            // No need to increment `i` since next vec item is already under `i`
        }
    }
}

void gc_cleanup() {
    gc_mark();
    gc_sweep();
}

void gc_init(size_t heap_limit_) {
    heap_limit = heap_limit_;
    heap_size = 0;
    allocations = vecNew();
    roots = vecNew();
}

void gc_deinit() {
#ifndef NDEBUG
    if (roots.size != 0) {
        fprintf(stderr, "[WARN] roots vec is nonempty before GC deinit. This may indicate a bug in program @ gc_deinit");
        exit(1);
    }
#endif

    for (int i = 0; i < allocations.size; ++i) {
        Allocation allocation = vecGet(allocations, i, Allocation);

        // There are no gc roots at this point, so all allocations are unreachable and are ready to be freed
        free(allocation.header);
    }

    vecDestroy(allocations);
    vecDestroy(roots);
}

void gc_push_root(Root root) {
    vecPush(roots, root, Root);
}

//void gc_pop_roots(size_t n) {
//    for (int i = 0; i < n; ++i) {
//        vecPop(roots, Root);
//    }
//}

Root gc_pop_root_unchecked() {
    return vecPop(roots, Root);
}

void gc_pop_root(Root root) {
    Root top_root = gc_pop_root_unchecked();
#ifndef NDEBUG
    if (top_root.object != root.object || top_root.visitor != root.visitor) {
        fprintf(stderr, "root pointer mismatch @ gc_pop_root");
        exit(1);
    }
#endif
}

// Variadic arguments should be of type Root
void gc_pop_roots(size_t count, ...) {
    va_list args;
    va_start(args, count); // read varidic arguments using `args` after parameter `count`

    for (int i = 0; i < count; ++i) {
        Root root = va_arg(args, Root);

        // Search for `root` in `roots` starting from end
        for (int j = (int) roots.size - 1; j > -1; --j) {
            Root *j_root = vecGetPointer(roots, j, Root);
            if (root.object == j_root->object && root.visitor == j_root->visitor) {
                // Found the matching root. Mark it as NULL.
                *j_root = make_root(NULL, NULL);
                goto found;
            }
        }
#ifndef NDEBUG
        // Root not found
        fprintf(stderr, "root not found @ gc_pop_roots");
        exit(1);
#endif

        found:
        (void) 0; // noop
    }

    va_end(args);

    // Now we should be able to pop `count` NULL roots from `roots` stack
    for (int i = 0; i < count; ++i) {
        Root root = gc_pop_root_unchecked();
#ifndef NDEBUG
        if (root.object != NULL || root.visitor != NULL) {
            fprintf(stderr, "expected a NULL root @ gc_pop_roots");
            exit(1);
        }
#else
        (void)root;
#endif
    }
}

ObjectP gc_alloc_full(size_t size, bool *is_gc_oom, bool *is_malloc_fail) {
    fprintf(stdout, "Requested allocation of %zu bytes. Current heap_size: %zu\n", size, heap_size);
    fflush(stdout);

    if (heap_size > heap_limit) {
        gc_cleanup();
        if (heap_size > heap_limit) {
            *is_gc_oom = true;
            return NULL;
        }
    }

    size_t allocation_size = header_size_in_allocation + size;

    GcHeaderP header = malloc(allocation_size);

    if (header == NULL) {
        *is_malloc_fail = true;
        return NULL;
    }

    vecPush(allocations, make_allocation(header, allocation_size), Allocation);

    // Write object header
    header->marked = false;

    heap_size += allocation_size;

    // Return pointer to requested object, skipping the GC header
    return gc_headerP_to_objectP(header);
}

ObjectP gc_alloc(size_t size) {
    bool is_gc_oom = false;
    bool is_malloc_failure = false;

    ObjectP object = gc_alloc_full(size, &is_gc_oom, &is_malloc_failure);

#ifndef NDEBUG
    if (object == NULL) {
        if (is_gc_oom) {
            // Could not free enough memory. Die
            fprintf(stderr, "OOM exception @ gc_alloc");
            exit(1);
        } else if (is_malloc_failure) {
            fprintf(stderr, "malloc failure when allocating a gc block @ gc_alloc");
            exit(1);
        } else {
            fprintf(stderr, "unreachable @ gc_alloc");
            exit(1);
        }
    }
#endif

    return object;
}

void gc_mark_object(ObjectP object) {
    objectP_to_gc_headerP(object)->marked = true;
}

size_t gc_get_limit() {
    return heap_limit;
}

void gc_set_limit(size_t limit) {
    heap_limit = limit;
}

size_t gc_get_heap_size() {
    return heap_size;
}
