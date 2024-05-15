#include <stdint.h>
#include <stdbool.h>

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