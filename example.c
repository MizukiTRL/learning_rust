#include <stdio.h>
int main(){
    int nums[5];

    while(1){
        int input;
        scanf("%d", &input);

        for (int i = 0; i< 5; i++){
            if(nums[i]==' \0'){
                nums[i] = input;
            }
            if(nums[i]==4){
                break;
            } 
        }
    }
}