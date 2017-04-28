/*
    The library provides a simple datastructure to access geolocated labels with an additional
    elimination time t and a label size factor. The library provides method to query a set of such
    labels with a bounding box and a minimum elimination time.
    
    Copyright (C) {2017}  {Filip Krumpe <filip.krumpe@fmi.uni-stuttgart.de}

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

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
