#include <iostream>
//https://yukicoder.me/problems/no/666
int main()
{
    long long int a = 0, b = 0;
    std::cin >> a >> b;

    long long int sum = 0;

    sum = (a % 1000000007)*(b%1000000007)%1000000007;

    std::cout << sum << std::endl;

    return 0;
}
