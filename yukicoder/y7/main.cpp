#include <iostream>
#include <vector>
#include <algorithm>
#include <numeric>
#include <map>
#include <cmath>
#include <string>

int main()
{
    int n = 0;
    std::cin >> n;

    //�?数を求め�?
    std::vector<int> primes;
    std::vector<int> tmp;

    for(int i = 2; i <= n; ++i )
    {
        tmp.push_back(i);
    }


    while( 1 )
    {
        if(tmp.size()==0)
            break;

        int i = tmp[0];
        
        primes.push_back(i);

        if(std::sqrt(n) <= i)
            break;

        std::vector<int> t;

        for(auto x: tmp){
            if(x%i != 0){
                t.push_back(x);
            }
        }

        tmp = t;
    }

    for(auto x: tmp){
        primes.push_back(x);
    }

    //勝ち�?けmapを作る
    std::map<int, bool> winlose;

    winlose[0] = false;
    winlose[1] = false;
    
    for( int i = 2; i <= n; ++i ){
        winlose[i] = false;
        
        for( auto const prime: primes)
        {
            if(prime > i ){
                break;
            }

            if(i-prime >= 2&& winlose[i-prime]==false){
              //  std::cout << "win[ " << i << " ] root: " << prime << std::endl;
              //  std::cout << "[i-prime]: " << winlose[i-prime] << std::endl;
                winlose[i] = true;
            }
        }
    }
    
    //for(auto i: primes)
     //   std::cout << i << std::endl;

    for(auto i: winlose){
        //std::cout << "winlose[ " << i.first << " ]: " << i.second << std::endl;
    }
    
    std::string wl = "Win";
    if(winlose[n]==false)
        wl = "Lose";

    std::cout << wl << std::endl;


    return 0;
}
