#include <string>
#include <iostream>
#include <set>
#include <map>
 
long long cnt(long long x){
    
    return x * (x-1) / 2;
}

int main(){
 
  long long int n = 0;
  std::cin >> n;
  

    std::map<char, long long int> ttmp;
  std::map<decltype(ttmp), long long > mp2;

  for(int i = 0; i < n ; ++i){
    
    std::map<char, long long int> mp;
  	std::string t;
    std::cin >> t;
    
    for(auto c: t){
      mp[c]++;
    }

    mp2[mp]++;
  
  }
  long long count = 0;
  for(auto m: mp2){
    	count += cnt(m.second);
  }
  
  std::cout << count << std::endl;
  
  return 0;
}