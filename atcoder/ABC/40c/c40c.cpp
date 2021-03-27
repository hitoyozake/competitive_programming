#include <iostream>
#include <string>
#include <vector>
#include <utility>

std::vector<std::pair<int, std::string>>  counting( std::string const & s ){
    int current = 0;
    int count = 1;
    std::vector<std::pair<int, std::string>> result;
    for(int i = 1; i < s.size(); ++i){
        if(s[current]!=s[i]){
            result.push_back({count, std::string(s[current], 1)});
            current = i;
            count = 1;
        }else{
            ++count;
        }
    }
    result.push_back({count, std::string(s[current], 1)});
    return result;
}

int main(){
    std::string s;

    std::cin >> s;
    //<<>>><<<<>>>>
    //>>><<>>>
    auto r = counting(s);

    for( auto i : r){
        std::cout << i.first << "," << i.second << std::endl;
    }

    return 0;
}