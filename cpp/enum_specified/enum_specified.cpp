// Purpose: ???
//

#include <cstdio>
#include <cstdint>
#include <cstring>

enum class A : int8_t
{
    Lowest = -2,
    Low = -1,
    Normal = 0,
    High = 1,
    Highest = 2,
};

enum B
{
    LOWEST = -2,
    LOW = -1,
    NORMAL = 0,
    HIGH = 1,
    HIGHEST = 2,
};

int main()
{
    A a = A::Lowest;
    B b = HIGHEST;


    printf("%i, %i\n", static_cast<int8_t>(a), static_cast<int8_t>(b));


    char bufA[sizeof(A)];
    memset(bufA, 1, sizeof(bufA));
    auto a2 = reinterpret_cast<A*>(bufA);

    char bufB[sizeof(B)];
    memset(bufB, 1, sizeof(bufB));
    auto b2 = reinterpret_cast<B*>(bufB);

    printf("%i, %i\n", static_cast<int8_t>(*a2), static_cast<int8_t>(*b2));

    *a2 = A();
    *b2 = B();

    printf("%i, %i\n", static_cast<int8_t>(*a2), static_cast<int8_t>(*b2));
}
