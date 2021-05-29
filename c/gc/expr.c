// A primitive Expression-tree interpreter example illustrating how to use the GC

#include "expr.h"

#include <stdio.h>

#include "gc.h"

// Select object visitor based on variable type
#define TYPE_TO_VISITOR(T) _Generic((T), \
Expression*: visit_expression             \
)

typedef struct {
    int v;
} Expression_Just;

typedef struct {
    struct Expression *a;
    struct Expression *b;
    char op;
} Expression_Binop;

typedef struct Expression {
    enum {
        JUST, BINOP
    } tag;
    union {
        Expression_Just Just;
        Expression_Binop Binop;
    };
} Expression;

// Function that gets called in Mark stage of a garbage collector
// It needs to mark the object as live and call transitive dependencies of the `expression` object
void visit_expression(ObjectP expression) {
    gc_mark_object(expression);

    Expression *e = expression;

    if (e->tag == JUST) {
        // Object does not depend on any references. Nothing to do
    } else if (e->tag == BINOP) {
        // Visit child expressions and mark them as live
        visit_expression(e->Binop.a);
        visit_expression(e->Binop.b);
    } else {
        fprintf(stderr, "unhandled expression tag @ visit_expression");
        exit(1);
    }
}

Expression *new_expression_just(int v) {
    Expression *e = gc_alloc(sizeof(Expression));
    GC_PUSH_ROOT(e); // Mark the allocation as a root so it won't be freed
    e->tag = JUST;
    e->Just.v = v;
    return e; // Return a rooted allocation
}

Expression *new_expression_binop(Expression *a, Expression *b, char op) {
    Expression *e = gc_alloc(sizeof(Expression));
    e->tag = BINOP;
    e->Binop.a = a;
    e->Binop.b = b;
    e->Binop.op = op;

    // a and b are now reachable from e. we don't need them to be roots anymore
    gc_pop_roots(2, MAKE_ROOT(a), MAKE_ROOT(b));

    // e is a new root
    GC_PUSH_ROOT(e);

    return e; // Return a rooted allocation
}

// Evaluate expression tree
int eval(Expression *e) {
    if (e->tag == JUST) {
        return e->Just.v;
    } else if (e->tag == BINOP) {
        int a = eval(e->Binop.a);
        int b = eval(e->Binop.b);
        if (e->Binop.op == '+') {
            return a + b;
        } else if (e->Binop.op == '*') {
            return a * b;
        } else {
            fprintf(stderr, "unhandled binop");
            exit(1);
        }
    } else {
        fprintf(stderr, "unhandled expression tag @ eval");
        exit(1);
    }
}

void example1() {
    for (int i = 0; i < 100; ++i) {
        // allocation returned by new_expression_binop is already rooted. no need to mark it here
        Expression *two_plus_three = new_expression_binop(
                new_expression_just(2),
                new_expression_just(3),
                '+'
        );

        // allocation returned by new_expression_binop is already rooted. no need to mark it here
        Expression *five_times_three = new_expression_binop(
                two_plus_three,
                new_expression_just(3),
                '*'
        );

        printf("val: %i\n", eval(five_times_three));

        printf("Hello, World!\n");

        fflush(stdout);

        // we don't need five_times_three anymore so it can be unrooted
        GC_POP_ROOT(five_times_three);
    }
}

// Unrolled version of example1. Makes it more clear how rooting gets transferred between variables.
void example2() {
    Expression *two = gc_alloc(sizeof(Expression));
    two->tag = JUST;
    two->Just.v = 2;

    GC_PUSH_ROOT(two);

    Expression *five = gc_alloc(sizeof(Expression));
    two->tag = JUST;
    two->Just.v = 5;

    GC_PUSH_ROOT(five);

    Expression *two_plus_two = gc_alloc(sizeof(Expression));
    two_plus_two->tag = BINOP;
    two_plus_two->Binop.a = two;
    two_plus_two->Binop.b = two;
    two_plus_two->Binop.op = '+';

    GC_PUSH_ROOT(two_plus_two);
    GC_POP_ROOT(two);

    Expression *four_times_five = gc_alloc(sizeof(Expression));
    two_plus_two->tag = BINOP;
    two_plus_two->Binop.a = two_plus_two;
    two_plus_two->Binop.b = five;
    two_plus_two->Binop.op = '*';

    GC_PUSH_ROOT(four_times_five);
    GC_POP_ROOT(five);

    printf("val: %i\n", eval(four_times_five));

    printf("Hello, World!\n");

    fflush(stdout);

    GC_POP_ROOT(four_times_five);
}

void test() {
    gc_init(1000);
    example1();
    gc_deinit();
}
