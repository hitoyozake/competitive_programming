#include <iostream>
#include <map>
#include <string>

using ll = long long;

int main(){

    ll n;
    std::cin >> n;

    std::map<char, ll> m;

    for(ll i = 0; i < n; ++i){
        std::string tmp;
        std::cin >> tmp;
        ++m[tmp[0]];
    }

  ll result = 0;
  //足していく
   // MARCH
    result += m['M']*m['A']*m['R'];
    result += m['M']*m['A']*m['C'];
    result += m['M']*m['A']*m['H'];

    result += m['M']*m['R']*m['C'];
    result += m['M']*m['R']*m['H'];

    result += m['M']*m['C']*m['H'];

    result += m['A']*m['R']*m['C'];
    result += m['A']*m['R']*m['H'];
    result += m['A']*m['C']*m['H'];
    
    result += m['R']*m['C']*m['H'];

    std::cout << result << std::endl;
    return 0;
}
