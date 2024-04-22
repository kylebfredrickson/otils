#include <stdbool.h>
#include <stdint.h>

int8_t select_8(bool cond, int8_t a, int8_t b)
{
    return (~(cond - 1) & a) | ((cond - 1) & b);
}

int16_t select_16(bool cond, int16_t a, int16_t b)
{
    return (~(cond - 1) & a) | ((cond - 1) & b);
}

int32_t select_32(bool cond, int32_t a, int32_t b)
{
    return (~(cond - 1) & a) | ((cond - 1) & b);
}

int64_t select_64(bool cond, int64_t a, int64_t b)
{
    return (~(cond - 1) & a) | ((cond - 1) & b);
}

bool equal_8(int8_t a, int8_t b)
{
    return !(a ^ b);
}

bool equal_16(int16_t a, int16_t b)
{
    return !(a ^ b);
}

bool equal_32(int32_t a, int32_t b)
{
    return !(a ^ b);
}

bool equal_64(int64_t a, int64_t b)
{
    return !(a ^ b);
}

int8_t compare_8(int8_t a, int8_t b)
{
    int16_t aa = (int16_t)a;
    int16_t bb = (int16_t)b;
    return ((aa - bb) >> 15) - ((bb - aa) >> 15);
}

int8_t compare_16(int16_t a, int16_t b)
{
    int32_t aa = (int32_t)a;
    int32_t bb = (int32_t)b;
    return ((aa - bb) >> 31) - ((bb - aa) >> 31);
}

int8_t compare_32(int32_t a, int32_t b)
{
    int64_t aa = (int64_t)a;
    int64_t bb = (int64_t)b;
    return ((aa - bb) >> 63) - ((bb - aa) >> 63);
}

int8_t compare_64(int64_t a, int64_t b)
{
    __int128_t aa = (__int128_t)a;
    __int128_t bb = (__int128_t)b;
    return ((aa - bb) >> 127) - ((bb - aa) >> 127);
}
