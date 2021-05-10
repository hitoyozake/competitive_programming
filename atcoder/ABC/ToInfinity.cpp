#include <iostream>
#include <string>

int main(){
  	std::string s;
  	std::cin >> s;
  	int k;
  	std::cin >> k;
  
  	//k文字目まで1だったら1)
  	if(s.size() >= k){
          bool flag = false;
          for(int i = 0; i < k; ++i){
              if(s[i]!='1'){
                  flag = true;
              }
          }
          if(!flag){
              std::cout << 1 << std::endl;
              return 0;
          }
    }

    //そうじゃない場合は1じゃな数字が出てきたらその文字
    for(auto i: s){
        if(i != '1'){
            std::cout << i << std::endl;
            return 0;
        }
    }
  
 	return 0;
  
}
