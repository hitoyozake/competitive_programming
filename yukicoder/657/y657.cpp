#include <iostream>
#include <map>
#include <vector>

int tetra(std::map<int, long long int> & map, long long int const n)
{
    if(n<=3)
        return 0;
    if(n==4)
        return 1;

    if(map.find(n) == map.end())
    {
        map[n] = (tetra(map, n-1) + tetra(map, n-2) + tetra(map, n-3) + tetra(map, n-4))%17;
        
    }

    return map[n];

}


int main()
{
    int num = 0;
    std::map<int, long long int> map;
    std::cin >> num;
    std::vector<int> v;
    for( int i = 0; i < num; ++i)
    {
        int x = 0;
        std::cin >> x;
        for(int j = 0; j < x; ++j)
        {
            tetra(map, j);
        }
        v.push_back(tetra(map, x));
    }

    for( auto i: v)
    {
        std::cout << i%17 << std::endl;
    }

    return 0;
}
