#include <iostream>
#include <map>

int main(){

    long long n = 0, k = 0;

    std::cin >> n >> k;

    std::map<int, long long int, std::less<int>> bucket;


    for(long long int i = 0; i < n; ++i){
        long long int tmp = 0, tmp_n = 0;
        std::cin >> tmp >> tmp_n;

        bucket[(int)tmp] += tmp_n;
    }

    long long int count = k;
    int index = 0;

    for(auto const & it: bucket){

        count += it.second;
        index =  it.first;
        if(count >= k)
            break;
   
    }

    std::cout << index << std::endl;  

    return 0;

}