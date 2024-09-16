#include <stdio.h>

int main()
{
    int nb1, nb2, quotient, reste;
    scanf("%d/%d", &nb1, &nb2);
    quotient = nb1 / nb2;
    reste = nb1 % nb2;

    printf("%d/%d donne q=%d et r=%d", nb1, nb2, quotient, reste);

    return 0;
}
