#include<stdio.h>

int main() {
   int a, b, sum;

   printf("\nEnter a number: ");
   scanf("%d", &a);
   printf("\nEnter another number: ");
   scanf("%d", &b);

   sum = a + b;

   printf("\nTotal : %d\n\n", sum);

   return(0);
}