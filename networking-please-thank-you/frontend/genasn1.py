#!/bin/python3

import sys
from sage.all import lcm

def gen_config(p, q):
	modulus = p * q
	version = 0
	pubExp = 65537
	privExp = pow(pubExp, -1, lcm(p - 1, q - 1))
	e1 = privExp % (p - 1)
	e2 = privExp % (q - 1)
	coeff = pow(q, -1, p)
	
	print('asn1=SEQUENCE:rsa_key')
	print('')
	print('[rsa_key]')
	print('version' + '=INTEGER:' + str(version))
	print('modulus' + '=INTEGER:' + str(modulus))
	print('pubExp' + '=INTEGER:' + str(pubExp))
	print('privExp' + '=INTEGER:' + str(privExp))
	print('p' + '=INTEGER:' + str(p))
	print('q' + '=INTEGER:' + str(q))
	print('e1' + '=INTEGER:' + str(e1))
	print('e2' + '=INTEGER:' + str(e2))
	print('coeff' + '=INTEGER:' + str(coeff))

args = sys.argv[1:]

p = ''
q = ''

set_q = False

for line in sys.stdin:
	if line == '\n':
		if set_q:
			break
		set_q = True
		continue
	
	if not set_q:
		p += line.strip().replace(':', '')
	else:
		q += line.strip().replace(':', '')

p = int(p, 16)
q = int(q, 16)

gen_config(p, q)
