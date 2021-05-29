#include <iostream>
#include <vector>
#include <array>

#include <algorithm>

#include "utils.hpp"
#include "radix_sort.hpp"

using namespace std;

namespace {
    // get a number using a window-bitmask
    // get(..., 8,  0) = 00000000000000000000000011111111
    // get(..., 8,  8) = 00000000000000001111111100000000
    // get(..., 8, 16) = 00000000111111110000000000000000
    // get(..., 8,  8) = 11111111000000000000000000000000
    size_t get(u32 number, size_t width, size_t offset) {
        u32 mask = ((1 << width) - 1) << offset;
//    std::cout << bitset<32>(mask) << std::endl;

        return (number & mask) >> offset;
    }
}

constexpr size_t NUMBER_WIDTH = sizeof(u32) * 8;
constexpr size_t STEP_WIDTH = 8;
constexpr size_t BUCKET_COUNT = 2 << STEP_WIDTH;

// Radix radix_sort for unsigned 32 bit numbers
void radix_sort(u32 *array, size_t size) {
    // iterate over 4 number windows
    for (int i = 0; i < NUMBER_WIDTH / STEP_WIDTH; ++i) {
        // count numbers to allocate memory in buckets
        ::array<size_t, BUCKET_COUNT> counts = {0};

        for (int j = 0; j < size; ++j) {
            counts[get(array[j], STEP_WIDTH, STEP_WIDTH * i)] += 1;
        }

        // buckets
        vector<u32> buckets[BUCKET_COUNT];

        // memory preallocation
        for (int j = 0; j < BUCKET_COUNT; ++j) {
            buckets[j].reserve(counts[j]);
        }

        // sort numbers into buckets
        for (int j = 0; j < size; ++j) {
            buckets[get(array[j], STEP_WIDTH, STEP_WIDTH * i)].push_back(array[j]);
        }

        // merge numbers back into array
        size_t nn = 0;
        for (int j = 0; j < BUCKET_COUNT; ++j) {
            for (int k = 0; k < buckets[j].size(); ++k) {
                array[nn] = buckets[j][k];
                nn++;
            }
        }
    }
}

void example1() {
    // Sorting 10 000 000 numbers
    // radix sort — 1.3 seconds
    // std::sort — 4.7 seconds

    std::cout << "Hello, World!" << std::endl;

    constexpr size_t N = 10'000'000;

    vector<u32> arr{};
    arr.reserve(N);

    random_number_generator r{};
    for (int i = 0; i < N; ++i) {
        arr.push_back(r.get_number());
    }

    Timer tm{};
    tm.start();
    radix_sort(arr.data(), arr.size()); // radix_sort
//    sort(arr.begin(), arr.end()); // STL std::sort
    tm.stop();
    cout << tm.elapsed_milliseconds();

//    for (auto &n:arr) {
//        std::cout << n << std::endl;
//    }
}