#include <toolkit/tcp.hpp>

#include "reporter.hpp"
#include "hooks.hpp"

extern "C" void megaton_main() {
    botw::tcp::init();
    botw::tcp::start_server(5001);
    botw::ist::trace::init();
    botw::ist::trace::install_hooks();
}

