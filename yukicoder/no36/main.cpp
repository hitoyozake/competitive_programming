#include <iostream>
#include <vector>
#include <map>
#include <cmath>
using i64 = long long;

int main()
{

    i64 n = 0;
    std::cin >> n;
    bool flag = false;
    const i64 max = 100000000000000;

    const i64 goal = i64(std::sqrt(n));
    int counter = 0;
    for( i64 i = 2; i <= goal; ++i){
        //std::cout << i << std::endl;
        if(n%i==0)
        {
            ++counter;
            if(counter>=3)
            {
                flag = true;
                break;
            }
            //std::cout << "i: " << i << std::endl;
            n /= i;
            i = 1;
            //std::cout << n << std::endl;
        }
    }

    if(n!=1)
        ++counter;

    if(counter>=3)
        flag=true;
    if(flag==false)
        std::cout << "NO" << std::endl;
    else
        std::cout << "YES" << std::endl;

    return 0;
}
