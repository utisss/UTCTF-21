OUT_DIR ?= .
DEBUG_DEFINITION ?= DEBUG
USE_OPENCL ?= n

CC = gcc
ifeq ($(USE_OPENCL),y)
CC_ARGS = -lcrypto -lOpenCl -Wall -Wextra -Werror -D$(DEBUG_DEFINITION) -DOPENCL
else
CC_ARGS = -lcrypto -Wall -Wextra -Werror -D$(DEBUG_DEFINITION)
endif

AR = ar

ifeq ($(USE_OPENCL),y)
OBJECTS = opencl_attack.o opencl_helper.o openssl_attack.o sha224_length_extension_attack.o private_sha224_length_extension_attack.o
else
OBJECTS = openssl_attack.o sha224_length_extension_attack.o private_sha224_length_extension_attack.o
endif

all: kernel.xxd $(OBJECTS)
	$(AR) rcs libsha224_length_extension_attack.a $(OBJECTS)

$(OBJECTS): %.o: %.c
	$(CC) $(CC_ARGS) -c $^

kernel.xxd: kernel.cl
	xxd -i < kernel.cl > kernel.xxd && echo ', 0' >> kernel.xxd

clean: 
	rm -f $(OUT_DIR)/*.o $(OUT_DIR)/*.a $(OUT_DIR)/*.xxd

.PHONY: all clean
