#include <iostream>
#include <string>
#include <set>
#include <queue>
#include <vector>

void print_state(std::vector<int> const & s){
    //std::cout << "print: " << std::endl;
    for( int i = 0; i < s.size()-1; ++i){
        std::cout << s[i] << ",";
    }

    std::cout << s.back() << std::endl;
}

int main()
{
    int n = 0;
    std::cin >> n;
    std::vector<int> coins;
    std::vector<int> init_state(n, 0);
    for( int i = 0; i < n; ++i )
    {
        int in = 0;
        std::cin >> in;
        coins.push_back(in);
    }

    for( int i = 0; i < n; ++i )
    {
        int in = 0;
        std::cin >> in;
        init_state[i] = in==1;
    }

    std::queue<std::vector<int>> states_q;

    std::set<std::vector<int>> states;
    states_q.push(init_state);
    
    bool found = false;
    int counter = 0;
    while(!states_q.empty()){
        //std::cout << "start" << std::endl;
        std::vector<int> const s = states_q.front();
        int t = 1;
        for(int i = 0; i < n; ++i )
        {
            t *= s[i];
        }

        if(t==1)
        {
            found=true;
            //states_q.clear();
            break;
        }
        
        //auto const r = states.insert(s);
        states_q.pop();
        if(states.find(s)==states.end())
        {
            states.insert(s);
        
        //if(r.second){
            //知って�?る盤面ではなかっ�?
            //�?番に盤面を生成してqueueに追�?する
            std::cout << "********poped State: *******  :  ";
            print_state(s);
            for(int i = 0; i < n; ++i)
            {
                int const plus = (i+coins[i])%n;
                int const minus = n + (i-coins[i])%n-1;
                std::cout << "plus:" << plus %n << std::endl;
                std::cout << "minus:" << minus %n << std::endl;
                std::vector<int> s_copy = s;
                //std::vector<int> s_copy(n, 0);
                
                s_copy[plus%n] = (int)!(s_copy[plus] == 1);
                if(plus!=minus)
                    s_copy[minus] = (int)!(s_copy[minus] == 1);
                print_state(s_copy);
                states_q.push(s_copy);
                
                //std::cout << "*******" << i << "************" << std::endl;
                
            }
            //std::cout << "end" << std::endl;
        }
        else
        {
            continue;
        }
    }

    if(found)
        std::cout << "Yes" << std::endl;
    else
        std::cout << "No" << std::endl;

    return 0;
}





