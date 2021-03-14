#include "private_sha224_length_extension_attack.h"
#include "openssl_attack.h"
#include <pthread.h>
#include <stdlib.h>
#include <string.h>
#include <sys/sysinfo.h>

size_t bin_search_possible_hashes(const unsigned char (*possible_hashes)[SHA224_DIGEST_LENGTH], size_t possible_hashes_len, const unsigned char hash[SHA224_DIGEST_LENGTH]) {
	size_t low, mid, high;
	low = 0;
	high = possible_hashes_len;
	
	while(high > low) {
		mid = low + ((high - low) / 2);
		
		int result = memcmp(hash, possible_hashes + mid, SHA224_DIGEST_LENGTH);
		
		if (result < 0) {
			high = mid;
		} else if (result > 0) {
			low = mid + 1;
		} else {
			return mid;
		}
	}
	
	return possible_hashes_len;
}

void state_search_openssl_section(const unsigned char (*possible_hashes)[SHA224_DIGEST_LENGTH], size_t possible_hashes_len, volatile uint32_t* pos, volatile uint32_t* exposed_state, SHA256_CTX* c_source, uint64_t first, uint64_t last, volatile char* done, const unsigned char* m, size_t m_len) {
	SHA256_CTX c;
	unsigned char md_guess[SHA256_DIGEST_LENGTH];
	size_t pos_loc;
	
	for (uint64_t guess_large = first; guess_large < last; guess_large += 1) {
		uint32_t guess = (uint32_t)guess_large;
		c = *c_source;
		c.h[7] = guess;
		
		SHA256_Update(&c, (const void*)m, m_len);
		SHA256_Final(md_guess, &c);
		
		pos_loc = bin_search_possible_hashes(possible_hashes, possible_hashes_len, md_guess);
		
		if (pos_loc < possible_hashes_len) {
			*pos = pos_loc;
			*exposed_state = guess;
			*done = 1;
			break;
		} else if (*done) {
			break;
		}
	}
}

struct search_openssl_wrapper {
	const unsigned char (*possible_hashes)[SHA224_DIGEST_LENGTH];
	volatile uint32_t* pos;
	volatile uint32_t* exposed_state;
	SHA256_CTX* c;
	uint64_t start;
	uint64_t stop;
	const unsigned char* m;
	size_t possible_hashes_len;
	size_t m_len;
	volatile char* done;
};

void *state_search_openssl_section_wrapper(void* args) {
	struct search_openssl_wrapper* a = (struct search_openssl_wrapper*)args;
	state_search_openssl_section(a->possible_hashes, a->possible_hashes_len, a->pos, a->exposed_state, a->c, a->start, a->stop, a->done, a->m, a->m_len);
	
	return NULL;
}

uint64_t state_search_openssl(const unsigned char (*possible_hashes)[SHA224_DIGEST_LENGTH], size_t possible_hashes_len, const unsigned char initial_state[SHA224_DIGEST_LENGTH], size_t prior_blocks, const unsigned char* m, size_t m_len) {
	int threads = get_nprocs();
	if (threads <= 0) exit(1);
	volatile char done = 0;
	volatile uint64_t result = 0xFFFFFFFFFFFFFFFF;
	volatile uint32_t* exposed_state = (volatile uint32_t*)&result;
	volatile uint32_t* pos = exposed_state + 1;
	if(!is_little_endian()){
		pos = (volatile uint32_t*)&result;
		exposed_state = pos + 1;
	}
	SHA256_CTX c;
	
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
	c.Nl = prior_blocks * 512;
	
	pthread_t* thread_list = malloc(sizeof(pthread_t) * threads);
	if(!thread_list) exit(1);
	struct search_openssl_wrapper* arg_wrappers = malloc(sizeof(struct search_openssl_wrapper) * threads);
	if(!arg_wrappers) exit(1);
	
	uint64_t step = 0x100000000 / threads;
	if (0xFFFFFFFF % threads) step += 1;
	for(size_t i = 0; i < (size_t)threads; i += 1) {
		uint64_t start = step * i;
		uint64_t stop = start + step;
		if (stop > 0x100000000) stop = 0x100000000;
		
		if(pthread_create(thread_list + i, NULL, &state_search_openssl_section_wrapper, arg_wrappers + i)) exit(1);
		
		arg_wrappers[i].possible_hashes = possible_hashes;
		arg_wrappers[i].possible_hashes_len = possible_hashes_len;
		arg_wrappers[i].pos = pos;
		arg_wrappers[i].exposed_state = exposed_state;
		arg_wrappers[i].c = &c;
		arg_wrappers[i].start = start;
		arg_wrappers[i].stop = stop;
		arg_wrappers[i].done = &done;
		arg_wrappers[i].m = m;
		arg_wrappers[i].m_len = m_len;
	}
	
	for(size_t i = 0; i < (size_t)threads; i += 1) {
		if(pthread_join(thread_list[i], NULL)) exit(1);
	}
	
	free(thread_list);
	free(arg_wrappers);
	
	return result;
}
