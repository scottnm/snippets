#include <cstdio>
#include <cstdint>

#define FOR(type, iter, start, end) \
    for (type iter = (start); iter < (end); ++iter)

int
main()
{
    FOR(uint8_t, i, 0, 10)
    {
        printf("%i, ", i);
    }
    printf("\n");
    return 0;
}

