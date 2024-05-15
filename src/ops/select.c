#include <stdint.h>
#include <stdbool.h>

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