#include "ack.hpp"

#include <vector>
#include <exception>

using namespace std;

struct state {
    int m;
    int n;
    char generator_state;

    state(int m, int n, char generator_state) : m(m), n(n), generator_state(generator_state) {}
};

// Compute Ackermann function without relying on recursive function calls
int ack(int m, int n) {
    int result = 0;
    vector<state> stack{};

    stack.emplace_back(m, n, 'A');

    while (!stack.empty()) {
        state local_state = stack.at(stack.size() - 1);

        switch (local_state.generator_state) {
            case 'A':
                if (local_state.m == 0) {
                    result = local_state.n + 1;
                    stack.pop_back();
                } else if (local_state.n == 0) {
                    stack.at(stack.size() - 1).generator_state = 'B';
                    stack.emplace_back(local_state.m - 1, 1, 'A');
                } else {
                    stack.at(stack.size() - 1).generator_state = 'B';
                    stack.emplace_back(local_state.m, local_state.n - 1, 'A');
                }
                break;
            case 'B':
                if (local_state.m == 0) {
                    throw exception();
                } else if (local_state.n == 0) {
                    stack.pop_back();
                } else {
                    stack.at(stack.size() - 1).generator_state = 'C';
                    stack.emplace_back(local_state.m - 1, result, 'A');
                }
                break;
            case 'C':
                if (local_state.m == 0) {
                    throw exception();
                } else if (local_state.n == 0) {
                    throw exception();
                } else {
                    stack.pop_back();
                }
                break;
        }
    }

    return result;
}
