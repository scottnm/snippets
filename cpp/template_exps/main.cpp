#include <cstdio>

constexpr
size_t
ConstVal()
{
    return 1;
}

template<typename TError>
constexpr
TError
GetDefault();

template<>
constexpr
int
GetDefault<int>()
{
    return 100;
}

template<size_t N, size_t M = ConstVal()>
struct TestA
{
    char narr[N];
    char marr[M];
};

template<
    typename TError,
    typename TError default = GetDefault<TError>()
    >
struct TestB
{
    TError e{default};
};

int
main()
{
    // A
    {
        TestA<10, 20> a1;
        TestA<10> a2;
    }

    // B
    {
        TestB<int, 0> b1;
        TestB<int> b2;
        printf("b1: %i\tb2: %i\n", b1.e, b2.e);
    }
}
