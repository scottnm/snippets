#include <chrono>
using namespace std::literals::chrono_literals;
#include <cstdio>
#include <cstdint>
#include <thread>

thread_local uint32_t t_count{0};

void
Incrementer(
    uint32_t threadId,
    uint32_t limit
    )
{
    for (uint32_t i = 0; i < limit; i += 1)
    {
        t_count += 1;
        printf("[%u] inc %u/%u\n", threadId, t_count, limit);
        std::this_thread::sleep_for(100ms);
    }
}

int
main()
{
    std::thread threads[] =
    {
        std::thread(Incrementer, 0, 10),
        std::thread(Incrementer, 1, 5),
        std::thread(Incrementer, 2, 20),
    };

    for (std::thread& t : threads)
    {
        t.join();
    }

    return 0;
}
