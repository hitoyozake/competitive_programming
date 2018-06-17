#include <algorithm>
#include <utility>
#include <string>
#include <iostream>
#include <vector>
#include <queue>

int main()
{
    int n = 0;

    std::cin >> n;

    std::priority_queue< std::pair<int, int>, std::vector< std::pair<int, int> >,
/*        std::function<bool(std::pair<int, int>, std::pair<int, int>)> > my([](auto const & a, auto const & b)->bool
        {
            if(a.first < b.first)
                return true;
            if(a.second < b.second)
                return true;
            return false;
        });*/
        std::greater<std::pair<int, int>>
        > my;

    std::vector<int> e;

    for( int i = 0; i < n; ++i )
    {
        int tmp;
        std::cin >> tmp;
        
        my.push({tmp, 0});
    }
    for( int i = 0; i < n; ++i )
    {
        int tmp;
        std::cin >> tmp;
        e.push_back(tmp/2);
    }

    int max_count_min = 1000000;

    for(int i = 0; i < n; ++i)
    {
        auto que = my;
        int max_count = 0;
        for(int j = 0; j < n; ++j){
            auto selected = que.top();
//            std::cout << selected.first << ", " << selected.second << std::endl;
            que.pop();
            selected.second++;
            selected.first += e[(i+j)%n];
            que.push(selected);
            if(max_count < selected.second)
                max_count = selected.second;
        }
        if(max_count_min > max_count)
            max_count_min = max_count;
    }

    std::cout << max_count_min << std::endl;

    return 0;

}
