#ifndef OPENCL_ATTACK_H
#define OPENCL_ATTACK_H
#include <stdint.h>
#include <stddef.h>
#include <openssl/sha.h>

uint64_t state_search_opencl(const unsigned char (*possible_hashes)[SHA224_DIGEST_LENGTH], size_t possible_hashes_len, const unsigned char initial_state[SHA224_DIGEST_LENGTH], size_t prior_blocks, const unsigned char* m, size_t m_len);

#endif
