#pragma once

#include <memory>
class HelloSayer {
public:
  HelloSayer();
  void sayHello() const;
};

std::unique_ptr<HelloSayer> new_hello_sayer();
