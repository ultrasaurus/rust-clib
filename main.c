#include <stdio.h>


int add(int a, int b) {
  return a + b;
}

int main() {
   int a, b, sum;

   printf("\nEnter a number: ");
   scanf("%d", &a);
   printf("\nEnter another number: ");
   scanf("%d", &b);

   // sum = a + b;
   sum = add(a,b);
   
   printf("\nTotal : %d\n\n", sum);

   return(0);
}