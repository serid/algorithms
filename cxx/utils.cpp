#include "utils.hpp"

void Timer::start() {
    m_StartTime = std::chrono::system_clock::now();
    m_bRunning = true;
}

void Timer::stop() {
    m_EndTime = std::chrono::system_clock::now();
    m_bRunning = false;
}

double Timer::elapsed_milliseconds() {
    std::chrono::time_point<std::chrono::system_clock> endTime;

    if (m_bRunning) {
        endTime = std::chrono::system_clock::now();
    } else {
        endTime = m_EndTime;
    }

    return std::chrono::duration_cast<std::chrono::milliseconds>(endTime - m_StartTime).count();
}

double Timer::elapsed_seconds() {
    return elapsed_milliseconds() / 1000.0;
}

u32 random_number_generator::get_number() {
    std::uniform_int_distribution uniform_dist(1, 1 << 31);
    return uniform_dist(*e1);
}

random_number_generator::~random_number_generator() {
    delete e1;
}