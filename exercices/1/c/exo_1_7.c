#include <stdio.h>

int fib(int i)
{
    switch (i)
    {
    case 0:
        return 0;
    case 1:
        return 1;
    default:
        return fib(i - 1) + fib(i - 2);
    }
}

int main()
{
    printf("%d", fib(1000));
}