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
      for(int i = 0; i < number_of_children; i++) {
        IntVec copy_vec{iter, tokens.end()};
        Node n{copy_vec};
        used_tokens += n.get_used_tokens();
        iter += n.get_used_tokens();
        children.push_back(n);
      }

      for(int i = 0; i< number_of_metadata; i++) {
        metadata.push_back(*iter);
        iter++;
      }
      used_tokens += number_of_metadata;
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

    auto get_number_of_childer() const {
      return number_of_children;
    }

    long get_indexed_sum() const {
      long sum{0};
      if(number_of_children == 0) {
        for(const auto& value: metadata) {
          sum += value;
        }
        return sum;
      }

      for(const auto index: metadata) {
        if(index == 0) {
          std::cerr << "index 0, cont\n";
          continue;
        }
        if(index - 1 < number_of_children) {
          const auto& kiddo = children[index - 1];
          sum += kiddo.get_indexed_sum();
        }
      }
      return sum;
    }

    void print_metadata() const {
      std::cout << "Numer of children " << number_of_children << " Metadata: ";
      for(auto& item: metadata) {
        std::cout << item << " "; 
      }
      std::cout << "\n";
      for(auto& item: children) {
        item.print_metadata();
      }
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
  root.print_metadata();
  std::cout << "Sum of metadata: " << root.get_sum_metadata() << std::endl;
  std::cout << "IndexedSum of metadata: " << root.get_indexed_sum() << std::endl;

  return EXIT_SUCCESS;
}
