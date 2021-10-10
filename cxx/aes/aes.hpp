#pragma once

#include <stdexcept>

#include "mat.hpp"

namespace aes {
    enum class AesKeySize {
        K128, K192, K256
    };

    static constexpr int key_size_to_number(AesKeySize key_size) {
        switch (key_size) {
            case AesKeySize::K128:
                return 128;
            case AesKeySize::K192:
                return 192;
            case AesKeySize::K256:
                return 256;
        }
    };

    template<AesKeySize key_size>
    struct aes {
        static constexpr int mat_length = []() {
            return key_size_to_number(key_size) / 32;
        }();
        static constexpr int round_number = []() {
            switch (key_size) {
                case AesKeySize::K128:
                    return 11;
                case AesKeySize::K192:
                    return 13;
                case AesKeySize::K256:
                    return 15;
            }
        }();
        static constexpr int number_of_round_keys_needed = []() {
            return round_number + 1;
        }();

        using aes_mat = mat::mat<4, 4>;
        using key_mat = mat::mat<mat_length, 4>;
        using key_schedule_mat = mat::mat<4 * number_of_round_keys_needed, 4>;

        static void generate_key_schedule(key_schedule_mat &key_schedule) {

        }

        static void encrypt(const aes_mat &plaintext, const key_mat &key, aes_mat &out_ciphertext) {

        }
    };

    template<AesKeySize key_size>
    struct A {

    };

    template<>
    struct A<AesKeySize::K128> {
        typedef aes<AesKeySize::K128> type;
    };

    template<>
    struct A<AesKeySize::K192> {
        typedef aes<AesKeySize::K192> type;
    };

    template<>
    struct A<AesKeySize::K256> {
        typedef aes<AesKeySize::K256> type;
    };

    void test1();
}