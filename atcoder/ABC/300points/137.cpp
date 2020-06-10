#include <string>
#include <iostream>
#include <set>
#include <map>
 
long long cnt(long long x){
    if(x==1){
        return x;
    }
    return x + cnt(x-1);
}

int main(){
 
  long long int n = 0;
  std::cin >> n;
  
  std::map<std::set<char>, long long int> mp;
  
  for(int i = 0; i < n ; ++i){
  	std::string t;
    std::cin >> t;
    
    std::set<char> cs;
    
    for(auto c: t){
      cs.insert(c);
    }
  
    mp[cs]++;
  }
  long long count = 0;
  for(auto m: mp){
    if(m.second >= 2){
    	count += cnt(m.second-1);
    }
  }
  
  std::cout << count << std::endl;
  
  return 0;
}