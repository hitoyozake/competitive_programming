#include <iostream>
#include <algorithm>
#include <string>

unsigned int get_digitlen(long long x){
	return std::to_string(x).length();
}

unsigned int  f(long long a, long long b){
	return std::max(get_digitlen(a), get_digitlen(b));
}

int main(){

	long long n = 0;

	std::cin >> n;

	long long end = n;
	unsigned int result = 1000000;
	for(long long i = 1; i*i <=end; ++i){
		auto const md = n%i;
		if(md == 0){
			result = std::min(result, f(i, n/i));
		}
	}
	std::cout << result << std::endl;
	return 0;
}