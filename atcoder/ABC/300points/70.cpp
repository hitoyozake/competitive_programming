#include <iostream>
#include <string>
 
int main(){
  int a, b, c, d;
  std::string s;
  std::cin >> s;
  a = s[0] - '0';
  b = s[1] - '0';
  c = s[2] - '0';
  d = s[3] - '0';
  
  std::string pm = "+-";
  int pmi[2] = {1, -1};
  
  for(int j = 0; j < 2; ++j)
  for(int k = 0; k < 2; ++k)
  for(int l = 0; l < 2; ++l){
   
  	const int sum = a + pmi[j]*b + pmi[k]*c + pmi[l]*d;
    
    if(sum == 7)
    {
       std::cout << a << pm[j] << b << pm[k] 
            << c << pm[l] << d << "=7" << std::endl;
       return 0;
    }
  }
  
  return 0;
}

