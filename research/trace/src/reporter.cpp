#include <cstdlib>
#include <cstdarg>
#include <cstdio>
#include <array>
#include <nn/os.h>
#include <nn/time.h>
#include <toolkit/mem/unique_ptr.hpp>
#include <toolkit/scoped_lock.hpp>
#include <toolkit/tcp.hpp>

#include "reporter.hpp"

namespace botw::ist::trace {

static constexpr u32 MAX_REPORTERS = 0xFF;

static Reporter default_reporter{};
static mem::unique_ptr<nn::os::MutexType> send_mutex = nullptr;
static mem::unique_ptr<nn::os::MutexType> array_mutex = nullptr;
static std::array<std::pair<u64, mem::unique_ptr<Reporter>>, MAX_REPORTERS> reporters = {std::make_pair(Reporter::INVALID_THREAD, nullptr)};

void init() {
    send_mutex = mem::make_unique<nn::os::MutexType>();
    nn::os::InitializeMutex(send_mutex.get(), false, 0);
    array_mutex = mem::make_unique<nn::os::MutexType>();
    nn::os::InitializeMutex(array_mutex.get(), false, 0);
}

void send_event(u64 thread_id, u32 level, const char* msg) {
    toolkit::ScopedLock lock { send_mutex.get() };

    // get the timestamp
    nn::time::PosixTime time { 0 };
    nn::Result r = nn::time::StandardUserSystemClock::GetCurrentTime(&time);
    if (r.IsFailure()) {
        time.time = 0;
    }

    botw::tcp::sendf("{{%x %x %x %s}}", time.time, thread_id, level, msg);
}

Reporter& current_reporter() {
    auto thread = nn::os::GetCurrentThread();
    if (thread == nullptr) {
        return default_reporter;
    }
    u64 id = nn::os::GetThreadId(thread);

    for (u32 i = 0; i < MAX_REPORTERS; i++) {
        auto& entry = reporters[i];
        if (entry.second == nullptr) {
            // not found, make a new one
            toolkit::ScopedLock lock { array_mutex.get() };
            if (reporters[i].second != nullptr) {
                // another thread beat us to it, try searching again
                continue;
            }
            auto new_reporter = mem::make_unique<Reporter>(id);
            reporters[i] = std::make_pair(id, std::move(new_reporter));
            return *reporters[i].second;
        }
        if (entry.first == id) {
            return *entry.second;
        }
    }

    return default_reporter;
}

void Reporter::sendf(const char* format, ...) const {
    char buffer[0x1000];
    va_list args;
    va_start(args, format);
    int size = vsnprintf(buffer, sizeof(buffer), format, args);
    va_end(args);
    if (size <= 0) {
        return;
    }
    buffer[sizeof(buffer) - 1] = '\0';
    send_event(thread_id, level, buffer);
}

}
