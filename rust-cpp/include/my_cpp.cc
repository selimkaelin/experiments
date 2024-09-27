#include "my_cpp.h"
#include <iostream>
#include <memory>

HelloSayer::HelloSayer() {}

void HelloSayer::sayHello() const {
  std::cout << "Hello from inside C++!" << std::endl;
}

std::unique_ptr<HelloSayer> new_hello_sayer() {
  return std::unique_ptr<HelloSayer>(new HelloSayer());
}
