#include <stdio.h>
#include "lib.h"

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