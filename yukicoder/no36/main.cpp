#include <iostream>
#include <vector>
#include <map>
#include <math>
using i64 = long long;

int main()
{
    std::vector<i64> primes;

    const i64 max = 100000000000000;

    std::vector<i64> tmp;

    for(i64 i = 2; i <= max; ++i )
    {
        tmp.push_back(i);
    } 

    auto f = tmp.first();

    std::vector<i64> tmp2;
    do{
        primes.push_back(f);
        for( auto const x: tmp)
        {
            if(x%f != 0){
                tmp2.push_back(x);
            }
        }
        tmp = tmp2;
        f = tmp.first();
    }while(f <= std::sqrt(max)+1);



    return 0;
}
