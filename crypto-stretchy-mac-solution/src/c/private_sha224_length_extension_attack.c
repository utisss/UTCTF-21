#include "private_sha224_length_extension_attack.h"
#include <stdint.h>
#ifdef OPENCL
#include "opencl_helper.h"
#endif

unsigned char is_little_endian() {
	uint16_t tmp = 0x0001;
	return ((unsigned char *)(&tmp))[0];
}

#ifdef OPENCL
unsigned char use_opencl() {
	cl_int err;

	cl_platform_id platform_id;
	cl_device_id device_id;

	if((err = clGetPlatformIDs(1, &platform_id, NULL)) != CL_SUCCESS) {
		if(err != CL_INVALID_VALUE) {
			printf("%s\n", opencl_errstr(err));
		} else {
			return 0;
		}
	}

	if((err = clGetDeviceIDs(platform_id, CL_DEVICE_TYPE_GPU, 1, &device_id, NULL)) != CL_SUCCESS) {
		if(err != CL_INVALID_VALUE && err != CL_DEVICE_NOT_FOUND) {
			printf("%s\n", opencl_errstr(err));
		} else {
			return 0;
		}
	}

	return 1;
}
#endif
