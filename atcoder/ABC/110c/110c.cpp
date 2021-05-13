#include <iostream>
#include <string>
#include <map>

int main(){
    std::string s1, s2;
    std::cin >> s1;
    std::cin >> s2;

    std::map<char, char> dict1, dict2;
    for(int i = 0; i < s1.size(); ++i){
        {
            const auto c = dict1[s1[i]];
            if(c == '\0'){
                dict1[s1[i]] = s2[i];
            }
            else if(c != s2[i]){
                std::cout << "No" << std::endl;
                return 0;
            }
        }
        {
            const auto c = dict2[s2[i]];
            if(c == '\0'){
                dict2[s2[i]] = s1[i];
            }
            else if(c != s1[i]){
                std::cout << "No" << std::endl;
                return 0;
            }
        }
    }

    std::cout << "Yes" << std::endl;

    return 0;
    
}