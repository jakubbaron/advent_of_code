#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>

using StringVec = std::vector<std::string>;
using IntVec = std::vector<int>;

auto split(const std::string& s, char delimiter)
{
   IntVec tokens;
   int token{0};
   std::istringstream tokenStream(s);
   while(tokenStream >> token) { 
      tokens.push_back(token);
   }
   return tokens;
}

class Node {
  public:
    auto get_used_tokens() const {
      return used_tokens;
    }

    Node(IntVec tokens) {
      auto iter = tokens.begin();
      number_of_children = *iter;
      iter++;
      int number_of_metadata = *iter;
      iter++;
      used_tokens += 2;
      //std::cout << "Creating children[" << number_of_children << "], metadata[" << number_of_metadata << "] VecLen: " << tokens.size();
      for(int i = 0; i < number_of_children; i++) {
        IntVec copy_vec{iter, tokens.end()};
        Node n{copy_vec};
        used_tokens += n.get_used_tokens();
        iter += n.get_used_tokens();
        children.push_back(n);
      }

      //std::cout << " Metadata: ";
      for(int i = 0; i< number_of_metadata; i++) {
        metadata.push_back(*iter);
        //std::cout << *iter << " ";
        iter++;
      }
      used_tokens += number_of_metadata;
      //std::cout << " Used tokens: " << used_tokens << "\n";
    }

    long get_sum_metadata() const { 
      long sum{0};
      for(const auto& value: metadata) {
        sum += value;
      }
      for(const auto& child: children) {
        sum += child.get_sum_metadata();
      }
      return sum;
    }
      
  private:
    int number_of_children = 0;
    int used_tokens = 0;
    std::vector<int> metadata;
    std::vector<Node> children;
};

int main(int argc, char** argv) {
  std::ifstream infile("input.txt");
  std::string line;
  IntVec tokens;
  while (std::getline(infile, line)) {
    tokens = split(line, ' '); 
  }
  Node root(tokens);
  std::cout << "Sum of metadata: " << root.get_sum_metadata() << std::endl;

  return EXIT_SUCCESS;
}
