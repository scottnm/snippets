#include <stdio.h>
#include <time.h>
#include <assert.h>

int main()
{
    time_t now = time(NULL);
    struct tm gmNow = {0};
    struct tm* res = gmtime_s(&now, &gmNow);
    printf("res==gmNow %i, now %llu\n", res==&gmNow, now);
    return 0;
}
