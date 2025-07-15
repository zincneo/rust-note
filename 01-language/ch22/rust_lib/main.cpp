#include <iostream>

extern "C" {
    int add_numbers(int a, int b);  // 声明 Rust 函数
}

int main() {
    int result = add_numbers(10, 20);
    std::cout << "Result from Rust: " << result << std::endl;
    return 0;
}
