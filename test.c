#include <stdio.h>
#include "rot26.h"

int main() {
	const char *hello = "hello!";
	printf("data before: %s\n", hello);
	const char *ciphered = rot26_cipher(hello);
	printf("ciphered: %s\n", ciphered);
	const char *deciphered = rot26_decipher(ciphered);
	printf("deciphered: %s\n", deciphered);
	return 0;
}
