int do_stuff(int x, int y)
{
    int a;
    int b;
    a = x + y;
    b = 4*x - 3*y;
    return a*b + 2;
}

int main()
{
    int x = 4;
    int y = 2;
    return do_stuff(x, y);
}
