#include <iostream>
#include <set>
#include <algorithm>

int LIMIT = 100000 + 1;


int main(){

    int n = 0;
    std::cin >> n;
    int min_count = 1000000;
    

    for(int i = 0; i <= n; ++i){
        int array[] = {i, n-i};
        int sn[] = {6, 9};
        auto count = 0;
        
        for(int j = 0; j < 2; ++j){
           
           while(array[j]>0){
            count += array[j]%sn[j];
            array[j] /= sn[j];
           }
        }

        min_count = std::min(count, min_count);
     }



    std::cout << min_count  << std::endl;

    return 0;

}

