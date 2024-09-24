#include <stdio.h>
#include <stdlib.h>

int main()
{
    FILE *file;
    file = fopen("exo_1_4.txt", "w");
    char nb;
    scanf(" %c", &nb);
    getchar();

    while (nb <= 57 && nb >= 48)
    {
        fprintf(file, "%d\n", nb - 48);
        scanf(" %c", &nb);
        getchar();
    }

    fclose(file);

    return 0;
}