#ifndef SHA224_LENGTH_EXTENSION_ATTACK_H
#define SHA224_LENGTH_EXTENSION_ATTACK_H
#include <openssl/sha.h>
#include <stdint.h>

unsigned char* sha224_extended_hash(const unsigned char initial_state[SHA224_DIGEST_LENGTH], SHA_LONG exposed_state, size_t prior_blocks, const unsigned char* m, size_t m_len, unsigned char md[SHA224_DIGEST_LENGTH]);
uint64_t sha224_length_extension_attack(const unsigned char (*possible_hashes)[SHA224_DIGEST_LENGTH], size_t possible_hashes_len, const unsigned char initial_state[SHA224_DIGEST_LENGTH], size_t prior_blocks, const unsigned char* m, size_t m_len);

#endif
