#include <stdint.h>
#include <stdbool.h>

void swap8(bool cond, uint8_t *a, uint8_t *b)
{
    uint8_t mask = ~((uint8_t)cond - 1);
    *a ^= *b;
    *b ^= *a & mask;
    *a ^= *b;
}