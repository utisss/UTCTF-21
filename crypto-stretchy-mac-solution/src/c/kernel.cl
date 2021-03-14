__constant uint K[64] = {
	0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
	0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
	0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
	0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
	0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
	0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
	0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
	0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
};

inline uint rotater32(uint val, uchar rot) {
	return rotate(val, 8 * (uint)sizeof(val) - rot);
}

inline uchar bigendian_uint8_eq(uint8 val1, uint8 val2) {
	int8 cmp = val1 == val2;
	return cmp.s0 & cmp.s1 & cmp.s2 & cmp.s3 & cmp.s4 & cmp.s5 & cmp.s6 & cmp.s7;
}

inline uchar bigendian_uint8_le(uint8 val1, uint8 val2) {
	uchar equal, greater;
	
	equal = 1;
	greater = 0;
	
	greater |= ((val1.s0 > val2.s0) & equal);
	equal &= val1.s0 == val2.s0;
	
	greater |= ((val1.s1 > val2.s1) & equal);
	equal &= val1.s1 == val2.s1;
	
	greater |= ((val1.s2 > val2.s2) & equal);
	equal &= val1.s2 == val2.s2;
	
	greater |= ((val1.s3 > val2.s3) & equal);
	equal &= val1.s3 == val2.s3;
	
	greater |= ((val1.s4 > val2.s4) & equal);
	equal &= val1.s4 == val2.s4;
	
	greater |= ((val1.s5 > val2.s5) & equal);
	equal &= val1.s5 == val2.s5;
	
	greater |= ((val1.s6 > val2.s6) & equal);
	equal &= val1.s6 == val2.s6;
	
	greater |= ((val1.s7 > val2.s7) & equal);
	equal &= val1.s7 == val2.s7;
	
	return !greater;
}

inline uint8 compress(__local uint schedule[64], uint8 state) {
	uint temp1, temp2, S0, S1, ch, maj;
	
	for(uint i = 0; i < 64; i += 1) {
		S1 = rotater32(state.s4, 6) ^ rotater32(state.s4, 11) ^ rotater32(state.s4, 25);
		ch = (state.s4 & state.s5) ^ ((~state.s4) & state.s6);
		temp1 = state.s7 + S1 + ch + K[i] + schedule[i];
		S0 = rotater32(state.s0, 2) ^ rotater32(state.s0, 13) ^ rotater32(state.s0, 22);
		maj = (state.s0 & state.s1) ^ (state.s0 & state.s2) ^ (state.s1 & state.s2);
		temp2 = S0 + maj;
		
		state.s7 = state.s6;
		state.s6 = state.s5;
		state.s5 = state.s4;
		state.s4 = state.s3 + temp1;
		state.s3 = state.s2;
		state.s2 = state.s1;
		state.s1 = state.s0;
		state.s0 = temp1 + temp2;
	}
	
	return state;
}

// possible_hashes_len > 0
inline uint bin_search_possible_hashes(__local uint8* possible_hashes, uint8 hash, uint possible_hashes_len) {
	uint v, ret;
	
	hash.s7 = 0;
	
	ret = sizeof(uint) - clz(possible_hashes_len - 1);
	v = (ret > (ret - 1));
	v = (v * (1 << (ret - 1))) + (!v); // 1 if possible_hashes_len == 1
	
	ret = bigendian_uint8_le(possible_hashes[v], hash) * (possible_hashes_len - v);
	while(v /= 2) {
		ret += bigendian_uint8_le(possible_hashes[ret + v], hash) * v;
	}
	
	return max(ret, !bigendian_uint8_eq(possible_hashes[ret], hash) * possible_hashes_len);
}

__kernel void compress_search(__constant uint8* possible_hashes, __local uint8* possible_hashes_loc, uint8 initial_state, __constant uint schedule[64], __local uint schedule_loc[64], __global uint pos_exposed_state[2], uint possible_hashes_len) {
	uint i;
	uint8 hash, state;
	size_t id, minimum_then_guess, max_non_inclusive;
	
	id = get_local_id(0);
	
	if(!id) {
		for(i = 0; i < 64; i += 1) {
			schedule_loc[i] = schedule[i];
		}
		for(i = 0; i < possible_hashes_len; i += 1) {
			possible_hashes_loc[i] = possible_hashes[i];
		}
	}
	barrier(CLK_LOCAL_MEM_FENCE);
	
	id = get_global_id(0);
	minimum_then_guess = get_global_size(0);
	max_non_inclusive = ((minimum_then_guess - 1) + 0x0100000000)/minimum_then_guess;
	minimum_then_guess = id * max_non_inclusive;
	max_non_inclusive = min((size_t)0x0100000000, minimum_then_guess + max_non_inclusive);
	
	for(; minimum_then_guess < max_non_inclusive; minimum_then_guess += 1) {
		uint guess = (uint)minimum_then_guess;
		state = initial_state;
		state.s7 = guess;
		
		i = bin_search_possible_hashes(possible_hashes_loc, compress(schedule_loc, state) + state, possible_hashes_len);
		
		if(i < possible_hashes_len) {
			pos_exposed_state[0] = i;
			pos_exposed_state[1] = guess;
			
			return;
		} else if(!(guess & 0xFFF) && pos_exposed_state[0] < 0xFFFFFFFF) {
			return;
		}
	}
}
