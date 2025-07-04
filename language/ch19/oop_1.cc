#include <iostream>
#include <vector>

// 封装和继承的概念不过多赘述，这里主要看c++中如何处理多态，以及继承导致的问题
class Base {
public:
  // 1. c++实现多态的方式
  // 虚函数
  virtual void func() {
    std::cout << "Base::func" << std::endl;
  }
  int val = 0;
};

class A: public Base {
public:
  void func() override {
    std::cout << "A::func" << std::endl;
  }
  int val = 10;
};

class B: public Base {
public:
  void func() override {
    std::cout << "B::func" << std::endl;
  }
  int val = 5;
};

int main() {
  A a;
  A a2;
  B b;

  // 编译器行为并非语言规定
  // 实现多态的方式是A,B的实例中包含的对象首地址其实是一张虚函数表
  std::vector<Base*> pArray = {&a, &b};
  for (auto pEle : pArray) {
    // 运行时通过虚函数表来确定具体调用哪个方法
    pEle->func();
  }

  // 拿到对象首地址
  // 这种做法本质上是语言的未定义行为，实质上只是编译器这么实现了
  // 所以实际开发中不要这么干
  void **aVptr = reinterpret_cast<void**>(&a);
  void **a2Vptr = reinterpret_cast<void**>(&a2);
  void **bVptr = reinterpret_cast<void**>(&b);

  // 解引用拿到虚表地址
  void *aVptr_addr = *aVptr;
  void *a2Vptr_addr = *a2Vptr;
  void *bVptr_addr = *bVptr;

  // 同一个类型，虚表地址相同
  std::cout << "a的虚表地址" << aVptr_addr << std::endl;
  std::cout << "a2的虚表地址" << aVptr_addr << std::endl;

  std::cout << "b的虚表地址" << bVptr_addr << std::endl;

  
  // 子类调用父类的同名虚函数
  (&a)->Base::func();

  // 同名属性的指定，继承链长了可能继承下来很多没必要的属性和方法
  std::cout << a.val << std::endl;
  std::cout << a.Base::val << std::endl;

  // 通过以上操作可以看到c++传统的实现多态的方式是通过运行时确定的，相比Rust中的零成本抽象开销会更大安全风险也更大
  return 0;
}
