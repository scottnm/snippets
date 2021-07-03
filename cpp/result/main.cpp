#include <cassert>
#include <cstdio>
#include <cstdint>
#include <utility>

typedef uint32_t MyError;
constexpr MyError c_mySuccces = 0;

template<typename TError>
constexpr
TError
success();

template<>
constexpr
MyError
success<MyError>()
{
    return c_mySuccces;
}

template<
    typename TValue,
    typename TError = MyError,
    typename TError success = success<TError>()
    >
class Result
{
public:
    Result(
        TValue v
        ) :
        v(std::move(v)),
        err(success)
    {
    }

    Result(
        TError err
        ) :
        err(err)
    {
    }

    bool
    Failed()
    {
        return err != success;
    }

    TError
    Error()
    {
        return err;
    }

    TValue&
    Value()
    {
        assert(!Failed());
        return v;
    }

private:
    TValue v;
    TError err;
};

class NonTrivialType
{
public:
    NonTrivialType() : i(0) {}
    NonTrivialType(
        NonTrivialType&& ntt
        )
    {
        int temp = ntt.i;
        ntt.i = 0;
        this->i = temp;
    }

    MyError
    Initialize(
        int i
        )
    {
        if (i % 2 == 0)
        {
            this->i = i;
            return c_mySuccces;
        }

        return 1;
    }

private:
    NonTrivialType(const NonTrivialType&)=delete;

    int i;
};

Result<NonTrivialType>
GetResultType(
    int i
    )
{
    // TODO: what-if Initialize returned a result type ._.
    NonTrivialType ntt;
    MyError e = ntt.Initialize(i);
    if (e == c_mySuccces)
    {
        return Result<NonTrivialType>(std::move(ntt));
    }
    return Result<NonTrivialType>{e};
}

#define VERIFY(someBool) \
    do { if (!(someBool)) \
    { \
        printf("Check failed (" #someBool ")! line: %i\n", __LINE__); \
    } } while (false)

void
TestResultType(
    int i
    )
{
    Result<NonTrivialType> res = GetResultType(i);
    if (i % 2 == 0)
    {
        VERIFY(!res.Failed());
        VERIFY(res.Error() == c_mySuccces);
        NonTrivialType& nttRef = res.Value();
    }
    else
    {
        VERIFY(res.Failed());
        VERIFY(res.Error() == 1);
#ifdef EXERCISE_ASSERT
        NonTrivialType& nttRef = res.Value();
#endif // EXERCISE_ASSERT
    }
}

int
main()
{
    for (int i = 0; i <= 5; i += 1)
    {
        printf("testing i == %i\n", i);
        TestResultType(i);
    }
    return 0;
};
