#pragma once
#include <megaton/prelude.h>

namespace botw::ist::trace {

/** Initialize the trace reporter */
void init();

/** Send a trace event */
void send_event(u64 thread_id, u32 level, const char* msg);

/** 
 * Thread-specific reporter for sending trace data to the trace server.
 */
class Reporter {


friend class LevelScope;
class LevelScope {

public:
    LevelScope(Reporter& reporter, const char* name) : reporter(&reporter) {
        reporter.sendf("%s", name);
        reporter.level++;
    }

    ~LevelScope() {
        reporter->level--;
    }

private:
    Reporter* reporter;
};

public:
    static constexpr u64 INVALID_THREAD = 0xFFFFFFFFFFFFFFFF;

    Reporter() = default;
    Reporter(u64 thread_id) : thread_id(thread_id) {}
    /** Send a message */
    void send(const char* msg) const;
    /** Send a formatted message */
    void sendf(const char* format, ...) const;

    /** Increment the level for this scope and send an enter message */
    LevelScope scope(const char* name) {
        return LevelScope(*this, name);
    }

    bool is_top() const {
        return level == 0;
    }

private:
    /** Thread id for this reporter */
    u64 thread_id = INVALID_THREAD;
    /** The nested level */
    u32 level = 0;
};


/** Get the reporter for the current thread */
Reporter& current_reporter();

}
