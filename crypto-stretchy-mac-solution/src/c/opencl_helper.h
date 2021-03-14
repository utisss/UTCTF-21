#ifndef OPENCL_HELPER_H
#define OPENCL_HELPER_H

#define CL_TARGET_OPENCL_VERSION 220

#include <CL/cl.h>
#include <stdio.h>

char* opencl_errstr(cl_int err);

#define ClError(x) {cl_int opencl_error_handler_macro_value; if((opencl_error_handler_macro_value = x) != CL_SUCCESS) {printf("%s\n", opencl_errstr(opencl_error_handler_macro_value)); exit(1);}}

#endif
