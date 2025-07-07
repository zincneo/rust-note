#include "stdio.h"

int* func() {
  int num = 10; // num是局部变量
  int* ref_num = &num;
  return ref_num;
}
// num在这里被丢弃，但是c中返回了创建num时的那个内存地址，这样就造成了悬垂指针，就是调用者得到了已经被操作系统回收掉的内存地址
// 这样造成的问题是os可能已经又将该空间分配给其他程序，而上层调用者却可以通过该指针修改该内存位置的值
// gcc编译c文件的时候并不做这种检查，因此对c的内存安全就要靠程序员自己把握

int main() {
  int* ref = func();
  printf("%d", *ref);
  return 0;
}
