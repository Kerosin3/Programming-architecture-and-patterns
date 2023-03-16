#include <cstddef>
#include <cstdio>
#include <cstring>
#include <gmock/gmock.h>
#include <gtest/gtest.h>
#include <vector>

class employee
{
    char name[80];

  public:
    void putname(char *name);
    void getname(char *bname);

  private:
    double wage;
};

void employee::putname(char *ename)
{
    printf("name length is %zu, name %s\n", strlen(ename), ename);
    strcpy(name, ename);
}
void employee::getname(char *bname)
{
    size_t length = snprintf(0, 0, "name is %s\n", name);
    snprintf(bname, length, "name is %s\n", name);
}

int main(int argc, char **argv)
{
    char name[80];
    employee emp1;
    //   char const *alex_name = "Alex";
    emp1.putname(strdup("Alex"));
    emp1.getname(name);
    fprintf(stdout, "::%s\n", name);
    ::testing::InitGoogleTest(&argc, argv);
    ::testing::InitGoogleMock(&argc, argv);

    return RUN_ALL_TESTS();
}
