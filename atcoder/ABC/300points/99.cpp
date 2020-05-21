#include <iostream>
#include <set>

int LIMIT = 100000 + 1;

std::set<long long> make_set(){

    long long x9 = 1;
    long long x6 = 1;
    std::set<long long> s;
    for(int i = 0; i < 30; ++i){
            s.insert(x9);
            s.insert(x6);
    }
    return s;
}


int main(){

    int n = 0;
    std::cin >> n;

    auto s = make_set();

    int count = 0;

    for(auto it = s.rbegin(); it != s.rend(); ++it){

        count += n / *it;
        n %= *it;
    }

    std::cout << count + n << std::endl;

    return 0;

}

