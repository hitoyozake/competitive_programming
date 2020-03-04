#include <iostream>
#include <string>
#include <utility>
#include <vector>
#include <tuple>


//(nex
std::tuple<std::vector<int>, std::string> parse(std::string const & s){
    std::vector<int> r;
    std::string rs;
    std::string tmp;
    for(auto const c : s){
        if( c >= '0' && c <= '9'){
            tmp += c;
        }
        else{
            r.push_back(std::stol(tmp));
            tmp = "";
            rs += c;
        }
    }
    r.push_back(std::stol(tmp));
    return {r, rs};
} 


int main(){
    std::string s;
    std::cin >> s;
    
    auto [nums, ops] = parse(s);

    int sum = nums[0];

    for(std::size_t i = 0; i < ops.length(); ++i){
        if(ops[i]=='*'){
          sum += nums[i+1];
        }
        if(ops[i]=='+'){
          sum *= nums[i+1];
        }
    }

    std::cout << sum << std::endl;
    
}


