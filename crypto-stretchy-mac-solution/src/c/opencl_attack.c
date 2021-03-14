#include "private_sha224_length_extension_attack.h"
#include "opencl_attack.h"
#include "opencl_helper.h"
#include <string.h>
#include <stdio.h>

#ifdef NDEBUG
const size_t LOG_BUF_LEN = 0;
#else
const size_t LOG_BUF_LEN = 64 * 64 * 64;
#endif

#define WORK_DIM_DEFINED 1
const size_t WORK_DIM = WORK_DIM_DEFINED;
const size_t global_work_offset[WORK_DIM_DEFINED] = {0};
const size_t global_work_size[WORK_DIM_DEFINED] = {256 * 112};
const size_t local_work_size[WORK_DIM_DEFINED] = {256};

const char KERNEL_SOURCE_BYTES[] = { 
	#include "kernel.xxd"
};

const char* KERNEL_SOURCE = &*KERNEL_SOURCE_BYTES;

char* kernel_name = "compress_search";

void pad_to_schedule(const unsigned char* m, size_t m_len, size_t pre_len, cl_uint schedule[64]) {
	unsigned char le = is_little_endian();
	
	size_t total_len = pre_len + m_len;
	size_t zeros = (64 - ((total_len + 9) % 64)) % 64;
	unsigned char* schedule_as_uchars = (unsigned char*) schedule;
	memcpy(schedule_as_uchars, m, m_len);
	schedule_as_uchars[m_len] = 0x80;
	memset(schedule_as_uchars + 1 + m_len, 0, zeros);
	uint64_t* padded_len_pos = (uint64_t*)(schedule_as_uchars + (m_len + zeros + 1));
	if(le) {
		*padded_len_pos = __builtin_bswap64(total_len * 8);
	} else {
		*padded_len_pos = total_len * 8;
	}
	
	if(le) {
		for(cl_uint i = 0; i < 16; i += 1) {
			schedule[i] = (cl_uint)__builtin_bswap32(schedule[i]);
		}
	}
}

// 0 < rot < 32
cl_uint rotr32(cl_uint val, unsigned char rot) {
	return (val >> rot) | (val << (8 * sizeof(val) - rot));
}

cl_uint m_schedule_helper_1(cl_uint w) {
	return rotr32(w, 7) ^ rotr32(w, 18) ^ (w >> 3);
}

cl_uint m_schedule_helper_2(cl_uint w) {
	return rotr32(w, 17) ^ rotr32(w, 19) ^ (w >> 10);
}

void expand_schedule(cl_uint schedule[64]) {
	for(cl_uint i = 16; i < 64; i += 1) {
		schedule[i] = schedule[i - 16] + m_schedule_helper_1(schedule[i - 15]) + schedule[i - 7] + m_schedule_helper_2(schedule[i - 2]);
	}
}

void prep_hash_vector(const unsigned char (*in)[SHA224_DIGEST_LENGTH], cl_uint8* out, size_t len) {
	for(size_t i = 0; i < len; i += 1) {
		out[i].s0 = *(((cl_uint*)(in + i)) + 0);
		out[i].s1 = *(((cl_uint*)(in + i)) + 1);
		out[i].s2 = *(((cl_uint*)(in + i)) + 2);
		out[i].s3 = *(((cl_uint*)(in + i)) + 3);
		out[i].s4 = *(((cl_uint*)(in + i)) + 4);
		out[i].s5 = *(((cl_uint*)(in + i)) + 5);
		out[i].s6 = *(((cl_uint*)(in + i)) + 6);
		out[i].s7 = 0;
	}
	
	if(is_little_endian()) {
		for(size_t i = 0; i < len; i += 1) {
			out[i].s0 = (cl_uint)__builtin_bswap32(out[i].s0);
			out[i].s1 = (cl_uint)__builtin_bswap32(out[i].s1);
			out[i].s2 = (cl_uint)__builtin_bswap32(out[i].s2);
			out[i].s3 = (cl_uint)__builtin_bswap32(out[i].s3);
			out[i].s4 = (cl_uint)__builtin_bswap32(out[i].s4);
			out[i].s5 = (cl_uint)__builtin_bswap32(out[i].s5);
			out[i].s6 = (cl_uint)__builtin_bswap32(out[i].s6);
		}
	}
}

uint64_t state_search_opencl(const unsigned char (*possible_hashes)[SHA224_DIGEST_LENGTH], size_t possible_hashes_len, const unsigned char initial_state[SHA224_DIGEST_LENGTH], size_t prior_blocks, const unsigned char* m, size_t m_len) {
	if (!possible_hashes_len) {
		return 0xFFFFFFFF;
	}
	
	cl_int err;
	
	cl_platform_id platform_id;
	cl_device_id device_id;
	
	cl_context_properties properties[3];
	cl_context context;
	cl_command_queue command_queue;
	
	cl_program program;
	cl_kernel kernel;
	
	cl_mem possible_hashes_dev, schedule_dev, pos_exposed_state_dev;
	cl_uint possible_hashes_len_cl;
	cl_uint schedule[64];
	cl_uint8* possible_hashes_cl = malloc(possible_hashes_len * sizeof(cl_uint8));
	if(!possible_hashes_cl) {printf("Out of memory.\n");exit(1);}
	cl_uint8 initial_state_cl;
	cl_uint pos_exposed_state[2] = {0xFFFFFFFF, 0xFFFFFFFF};
	
	
	ClError(clGetPlatformIDs(1, &platform_id, NULL))
	ClError(clGetDeviceIDs(platform_id, CL_DEVICE_TYPE_GPU, 1, &device_id, NULL))
	
	// context properties list - must be terminated with 0
	properties[0] = CL_CONTEXT_PLATFORM;
	properties[1] = (cl_context_properties) platform_id;
	properties[2] = 0;
	
	// create context with GPU device
	context = clCreateContext(properties, 1, &device_id, NULL, NULL, &err);
	ClError(err)
	
	// create command queue using the context and device
	command_queue = clCreateCommandQueueWithProperties(context, device_id, NULL, &err);
	ClError(err)
	
	// create program from kernel source
	program = clCreateProgramWithSource(context, 1, &KERNEL_SOURCE, NULL, &err);
	ClError(err)
	if((err = clBuildProgram(program, 1, &device_id, NULL, NULL, NULL)) != CL_SUCCESS) {
		printf("Error building program\n");
		
		char buffer[LOG_BUF_LEN];
		size_t length;
		
		ClError(clGetProgramBuildInfo(program, device_id, CL_PROGRAM_BUILD_LOG, sizeof(buffer), buffer, &length))
		
		printf("%s\n", buffer);
		exit(1);
	}
	// specify kernel to execute
	kernel = clCreateKernel(program, kernel_name, &err);
	ClError(err)
	
	possible_hashes_len_cl = possible_hashes_len;
	pad_to_schedule(m, m_len, prior_blocks * 64, schedule);
	expand_schedule(schedule);
	prep_hash_vector(possible_hashes, possible_hashes_cl, possible_hashes_len);
	prep_hash_vector((const unsigned char (*)[SHA224_DIGEST_LENGTH])initial_state, &initial_state_cl, 1);
	
	possible_hashes_dev = clCreateBuffer(context, CL_MEM_READ_ONLY | CL_MEM_COPY_HOST_PTR | CL_MEM_HOST_WRITE_ONLY, possible_hashes_len * sizeof(*possible_hashes_cl), possible_hashes_cl, &err);
	ClError(err)
	
	schedule_dev = clCreateBuffer(context, CL_MEM_READ_ONLY | CL_MEM_COPY_HOST_PTR | CL_MEM_HOST_WRITE_ONLY, sizeof(schedule), schedule, &err);
	ClError(err)
	
	pos_exposed_state_dev = clCreateBuffer(context, CL_MEM_READ_WRITE | CL_MEM_COPY_HOST_PTR, sizeof(pos_exposed_state), pos_exposed_state, &err);
	ClError(err)
	
	ClError(clSetKernelArg(kernel, 0, sizeof(cl_mem), &possible_hashes_dev))
	ClError(clSetKernelArg(kernel, 1, possible_hashes_len * sizeof(*possible_hashes_cl), NULL))
	ClError(clSetKernelArg(kernel, 2, sizeof(initial_state_cl), &initial_state_cl))
	ClError(clSetKernelArg(kernel, 3, sizeof(cl_mem), &schedule_dev))
	ClError(clSetKernelArg(kernel, 4, sizeof(schedule), NULL))
	ClError(clSetKernelArg(kernel, 5, sizeof(cl_mem), &pos_exposed_state_dev))
	ClError(clSetKernelArg(kernel, 6, sizeof(possible_hashes_len_cl), &possible_hashes_len_cl))
	
	clEnqueueNDRangeKernel(command_queue, kernel, WORK_DIM, global_work_offset, global_work_size, local_work_size, 0, NULL, NULL);
	ClError(err)
	
	ClError(clEnqueueReadBuffer(command_queue, pos_exposed_state_dev, 1, 0, sizeof(pos_exposed_state), pos_exposed_state, 0, NULL, NULL))
	
	// cleanup - release OpenCL resources
	ClError(clReleaseMemObject(possible_hashes_dev))
	ClError(clReleaseMemObject(schedule_dev))
	ClError(clReleaseMemObject(pos_exposed_state_dev))
	ClError(clReleaseKernel(kernel))
	ClError(clReleaseProgram(program))
	ClError(clReleaseCommandQueue(command_queue))
	ClError(clReleaseContext(context))
	free(possible_hashes_cl);
	
	uint64_t result;
	if(is_little_endian()) {
		result = (uint64_t)pos_exposed_state[0];
		result <<= 32;
		result |= (uint64_t)pos_exposed_state[1];
	} else {
		result = *((uint64_t*)pos_exposed_state);
	}
	
	return result;
}
