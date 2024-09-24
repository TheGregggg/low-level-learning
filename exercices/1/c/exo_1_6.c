#include <stdio.h>

void p(int nb)
{
    if (nb < 10)
    {
        printf("  %d", nb);
    }
    else if (nb < 100)
    {
        printf(" %d", nb);
    }

    else
    {
        printf("%d", nb);
    }
}

int main()
{
    int max = 12;
    for (int i = 1; i <= max; i += 3)
    {
        for (int j = 1; j <= max; j++)
        {
            for (int k = 0; k < 3; k++)
            {
                printf("|  ");
                p(i + k);
                printf(" * ");
                p(j);
                printf(" = ");
                p((i + k) * j);
                printf("  ");
            }
            printf("|\n");
        }
        printf("\n");
    }
    return 0;
}