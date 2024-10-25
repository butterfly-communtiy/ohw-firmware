#ifndef BASE58_H
#define BASE58_H

#include <stdio.h>
#include <stdbool.h>
#include <stddef.h>

extern bool (*b58_sha256_impl)(void *, const void *, size_t);

extern bool b58tobin(void *bin, size_t *binsz, const char *b58, size_t b58sz);
extern int b58check(const void *bin, size_t binsz, const char *b58, size_t b58sz);

extern bool b58enc(char *b58, size_t *b58sz, const void *bin, size_t binsz);
extern bool b58check_enc(char *b58c, size_t *b58c_sz, uint8_t ver, const void *data, size_t datasz);
bool b58check_enc_rel(char *b58c, size_t *b58c_sz, uint8_t ver, const void *data, size_t datasz);
#endif
