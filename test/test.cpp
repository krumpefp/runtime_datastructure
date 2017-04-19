// compile with
// $ clang++ test.cpp -o test ../target/release/libruntime_datastructure.so

#include <iostream>

extern "C" {
  void* init(const char* input_path);
  
  bool is_good(void* ds);
}

int main(int argc, char** argv)
{
  if (argc < 2) {
    std::cout << "Please specify a c.e file" << std::endl;
    return 1;
  }
  
  std::cout << "Initializing the data structure from " << argv[1] << std::endl;
  void *ds = init(argv[1]);
  
  if (is_good(ds)) {
    std::cout << "Datastructure was created successfully!" << std::endl;
  } else {
    std::cout << "Failed to create datastructure!" << std::endl;
  }
  
  std::cout << "Press any key to continue ..." << std::endl;
  
  char c;
  std::cin >> c;
}
