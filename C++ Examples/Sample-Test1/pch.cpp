//
// pch.cpp
//

#include "pch.h"

long long factoriel(unsigned int n) {
	return n <= 1 ?
		1 :
		n * factoriel(n - 1);
}