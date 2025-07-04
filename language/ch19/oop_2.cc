#include <iostream>
#include <vector>
#include <functional>

// 面向对象的需求
// 有不同的对象要实现相同的行为
#if 0
// 1. 传统做法是通过继承不可以实例化的接口类(除了构造和析构函数只包含纯虚函数)
class Drawable {
public:
  virtual void draw() = 0; // 纯虚函数，子类必须重写
  virtual ~Drawable() = default;

protected:
  Drawable() = default;
};

class Circle : public Drawable {
public:
  void draw() override { std::cout << "Draw circle" << std::endl; }
};
class Rectangle : public Drawable {
public:
  void draw() override { std::cout << "Draw reactangle" << std::endl; }
};
#else
// 2. 类型擦除
// 通过std::function和模板捕获任意可调用对象，解耦接口和实现
class Drawable {
private:
  std::function<void()> draw_fn; // 存储任意可以drawable的对象
public:
  template <typename T> Drawable(T obj) : draw_fn([=] { obj.draw(); }) {}
  void draw() const { draw_fn(); }
};
class Circle {
public:
  void draw() const { std::cout << "Draw circle" << std::endl; }
};
class Rectangle {
public:
  void draw() const { std::cout << "Draw reactangle" << std::endl; }
};
#endif

int main() {

#if 0
  // 1. 传统虚函数表形式
  Circle c;
  Rectangle r;
  std::vector<Drawable*> vec = {&c, &r};
  for (auto e: vec) {
    e->draw();
  }
#else
  // 2. 类型参数形式
  Circle c;
  Rectangle r;
  // 多态通过类型Drawable消除实际调用的类型
  std::vector<Drawable> vec = {Drawable(c), Drawable(r)};
  for (auto e: vec) {
    e.draw();
  }
#endif
  return 0;
}
