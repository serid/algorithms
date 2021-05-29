#include <chrono>
#include <random>

#include <cstdint>

typedef uint32_t u32;

class Timer {
public:
    void start();

    void stop();

    double elapsed_milliseconds();

    double elapsed_seconds();

private:
    std::chrono::time_point<std::chrono::system_clock> m_StartTime;
    std::chrono::time_point<std::chrono::system_clock> m_EndTime;
    bool m_bRunning = false;
};

constexpr size_t uint_power(size_t base, size_t n) {
    size_t r = 1;
    for (int i = 0; i < n; ++i) {
        r *= base;
    }
    return r;
}

class random_number_generator {
    std::default_random_engine *e1;

public:
    random_number_generator() : e1(new std::default_random_engine(std::random_device()())) {}

    u32 get_number();

    ~random_number_generator();
};