#!/bin/bash

TMP="$(openssl genrsa 4096 | openssl rsa -text -noout)"
A="$(echo "$TMP" | sed -n '/prime1:/,/prime2:/p' | head -n -1 | tail -n +2)"
B="$(echo "$TMP" | sed -n '/prime2:/,/exponent1:/p' | head -n -1 | tail -n +2)"
C="$(openssl genrsa 4096 | openssl rsa -text -noout | sed -n '/prime1:/,/prime2:/p' | head -n -1 | tail -n +2)"

echo -e "${A}\n\n${B}\n" | ./genasn1.py > tmp.txt
openssl asn1parse -genconf tmp.txt -out tmp.der
openssl rsa -inform der -in tmp.der -out a.pem
mv a.pem a.key

echo -e "${B}\n\n${C}\n" | ./genasn1.py > tmp.txt
openssl asn1parse -genconf tmp.txt -out tmp.der
openssl rsa -inform der -in tmp.der -out b.pem
mv b.pem b.key

rm tmp.txt tmp.der

openssl req -new -key a.key -subj /CN=$1 -out a.csr
openssl req -new -key b.key -subj /CN=$1 -out b.csr
openssl genrsa -out account.key 4096

openssl rsa -in account.key -pubout
cat a.csr
cat b.csr

echo 'https://gethttpsforfree.com/'
