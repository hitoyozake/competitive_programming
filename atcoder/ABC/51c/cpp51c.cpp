#include <iostream>
#include <string>



int main(){
    int x1, y1, x2, y2;
    std::string s;
    std::cin >> x1 >> y1 >> x2 >> y2;

    int x = x2 - x1;
    int y = y2 - y1;

    for(int i = 0; i < y; ++i)
        s += "U";
    for(int i = 0; i < x; ++i)
        s += "R";

    for(int i = 0; i < y; ++i)
        s += "D";
    for(int i = 0; i < x; ++i)
        s += "L";

    // go 2
    s+= "L";
    for(int i = 0; i < y+1; ++i)
        s+= "U";
    for(int i = 0; i < x+1; ++i)
        s+= "R";
    s+=  "D";

    //return 1
    s+=  "R";
    for(int i = 0; i < y+1; ++i)
        s+=  "D";
    for(int i = 0; i < x+1; ++i)
        s+=  "L";
    s+=  "U";
    

    std::cout << s << std::endl;
   // std::cout << s.size() << std::endl;
    return 0;
    //UURDDLLUUURRDRDDDLLU
}