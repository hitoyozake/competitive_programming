#include <iostream>
#include <algorithm>
#include <string>

void debug(int a, int b, int tmp, int tmppre){
  std::cout << "a: " << a << ", b:" << b << ", tmp:" << tmp << ", tmppre: " << tmppre << std::endl;
}

int main(){

  int n = 0;

  std::cin >> n;

  int a = 0, b = 0;
  //aは前に取った場合，bは前に取らなかった場合
  int tmppre = 0; //1つ前の値
  std::string s;
  int gmax = 0, ngmax = 0;
  int pregmax = 0;
  for( int i = 0; i < n; ++i ){
    int tmp = 0;
    std::cin >> tmp;

    //取った場合の最大値
    //取らなかった場合の最大値
    // 取った場合の最大値 < 取らなかった場合の最大値
    //  -> 取った場合の最大値に取らなかった場合maxを代入
    // 取る場合は取らなかった場合最大値に取った値を代入
    // 取らない場合はとった場合最大値で代入

  gmax = ngmax + tmp;

  ngmax = std::max(pregmax, ngmax);

  if(gmax < ngmax){
    gmax = ngmax;
  }

  pregmax = gmax;



  }

  int out = std::max(gmax, pregmax);

  std::cout << out << std::endl;

  return 0;
}
