
#include <iostream>
#include <map>
#include <algorithm>

int main(){

    long long n;
    
    std::map<long long, long long> m;

    std::cin >> n;

    for(long long i = 0; i < n; ++i ){

        long long tmp;
        std::cin >> tmp;
        m[tmp] += 1;
    }

    auto count = std::count_if(m.begin(), m.end(), [](std::pair<long long, long long> const & x){ return x.second % 2 == 1; });

    std::cout << count << std::endl;

    return 0;

}

