#include <iostream>
#include <algorithm>
#include <utility>
#include <vector>

using ll = long long;

int main()
{
    ll n = 0, x;
    std::cin >> n >> x;
    
    std::vector<ll> points;
    points.push_back(x);
    for( ll i = 0; i < n; ++i ){
        ll tmp = 0;
        std::cin >> tmp;
        points.push_back(tmp);
    }

    std::sort(points.begin(), points.end());

    std::vector<ll> sub;
    sub.resize(points.size()-1);

    //最小の値を得る
    ll min = 10000000000 + 1;
    for(ll i = 1; i < points.size(); ++i){
        sub[i-1] =points[i] - points[i-1];
        //        sub.push_back(points[i] - points[i-1]);
    }

    std::sort(sub.begin(), sub.end());

    auto minv = sub[0];
    //std::cout << minv  << std::endl;
    for(auto i: sub){
        if(i%minv){
            std::cout << 1 << std::endl;
            return 0;
        }
    }
    std::cout << minv << std::endl;
    return 0;
}