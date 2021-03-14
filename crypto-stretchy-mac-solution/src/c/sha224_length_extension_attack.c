#include "sha224_length_extension_attack.h"
#include "private_sha224_length_extension_attack.h"
#include "openssl_attack.h"
#ifdef OPENCL
#include "opencl_attack.h"
#endif
#include <openssl/sha.h>
#include <stdint.h>
#include <string.h>
#include <assert.h> 
#include <stdlib.h>

unsigned char* sha224_extended_hash(const unsigned char initial_state[SHA224_DIGEST_LENGTH], SHA_LONG exposed_state, size_t prior_blocks, const unsigned char* m, size_t m_len, unsigned char md[SHA224_DIGEST_LENGTH]) {
	SHA256_CTX c;
	unsigned char md_tmp[SHA256_DIGEST_LENGTH];
	
	SHA256_Init(&c);
	if(is_little_endian()) {
		c.h[0] = __builtin_bswap32(*(((SHA_LONG*)initial_state) + 0));
		c.h[1] = __builtin_bswap32(*(((SHA_LONG*)initial_state) + 1));
		c.h[2] = __builtin_bswap32(*(((SHA_LONG*)initial_state) + 2));
		c.h[3] = __builtin_bswap32(*(((SHA_LONG*)initial_state) + 3));
		c.h[4] = __builtin_bswap32(*(((SHA_LONG*)initial_state) + 4));
		c.h[5] = __builtin_bswap32(*(((SHA_LONG*)initial_state) + 5));
		c.h[6] = __builtin_bswap32(*(((SHA_LONG*)initial_state) + 6));
	} else {
		c.h[0] = *(((SHA_LONG*)initial_state) + 0);
		c.h[1] = *(((SHA_LONG*)initial_state) + 1);
		c.h[2] = *(((SHA_LONG*)initial_state) + 2);
		c.h[3] = *(((SHA_LONG*)initial_state) + 3);
		c.h[4] = *(((SHA_LONG*)initial_state) + 4);
		c.h[5] = *(((SHA_LONG*)initial_state) + 5);
		c.h[6] = *(((SHA_LONG*)initial_state) + 6);
	}
	c.h[7] = exposed_state;
	c.Nl = prior_blocks * 512;
	
	SHA256_Update(&c, (const void*)m, m_len);
	SHA256_Final(md_tmp, &c);
	
	memcpy(md, md_tmp, SHA224_DIGEST_LENGTH);
	
	return md;
}

uint64_t sha224_length_extension_attack(const unsigned char (*possible_hashes)[SHA224_DIGEST_LENGTH], size_t possible_hashes_len, const unsigned char initial_state[SHA224_DIGEST_LENGTH], size_t prior_blocks, const unsigned char* m, size_t m_len) {
	assert(possible_hashes_len < 0x0000000100000000);
	assert(m_len <= 64 - 9); // only allow a single block for message plus padding (message schedule needs to be calculated once)
	
	uint64_t result;

	#ifdef OPENCL
	if (!use_opencl()) {
		result = state_search_openssl(possible_hashes, possible_hashes_len, initial_state, prior_blocks, m, m_len);
	} else {
		result = state_search_opencl(possible_hashes, possible_hashes_len, initial_state, prior_blocks, m, m_len);
	}
	#else
	result = state_search_openssl(possible_hashes, possible_hashes_len, initial_state, prior_blocks, m, m_len);
	#endif
	
	return result;
}
