#include <iostream>
#include <fstream>
#include <memory>
#include <vector>
#include <map>

class Task;
using STask = std::shared_ptr<Task>;

class Task {
  public:
    Task(const std::string& id):
      m_id(id),
      m_finished(false) {

    }

    void add_prerequisitive(STask stask) {
      m_prerquisitves.push_back(stask);
    }

    const auto is_finished() const {
      return m_finished;
    }
    const auto get_id() const {
      return m_id;
    }

    auto can_be_finished() const {
      for(const auto& item: m_prerquisitves) {
        if(!item->is_finished()) {
          return false;
        }
      }
      m_finished = true;
      std::cout << m_id;
      return true;
    }

    void print_prerequisitives() const {
      std::cout << "Item's [" << m_id << "] prerequisitves: ";
      for(const auto& item: m_prerquisitves) {
        std::cout << item->get_id() << " ";
      }
      std::cout << std::endl;
    }

  private:
    std::string m_id;
    mutable bool m_finished;
    std::vector<STask> m_prerquisitves;
};

int main(int argc, char** argv) {
  std::ifstream infile("input.txt");
  std::string line;
  std::map<std::string, STask> map_id;

  while (std::getline(infile, line)) {
    auto first_task_id = line.substr(5,1);
    auto prerequisitve_pos = line.find("step ");
    auto second_task_id = line.substr(prerequisitve_pos + 5, 1);

    STask first_task;
    if(!map_id.count(first_task_id)) {
      map_id.emplace(std::make_pair(first_task_id, std::make_shared<Task>(first_task_id)));
    }
    first_task = map_id[first_task_id];
    STask second_task;
    if(!map_id.count(second_task_id)) {
      map_id.emplace(std::make_pair(second_task_id, std::make_shared<Task>(second_task_id)));
    }
    second_task = map_id[second_task_id];
    second_task->add_prerequisitive(first_task);  
  }

  for(const auto& item: map_id) {
    item.second->print_prerequisitives();
  }

  bool all_finished = false;
  while(!all_finished) {
    all_finished = true;
    for(const auto& item: map_id) {
      const auto& stask = item.second;

      if(!stask->is_finished()) {
        if(stask->can_be_finished()) {
          all_finished = false;
          break;
        }
      }

      all_finished &= stask->is_finished();
    }
  }

  return EXIT_SUCCESS;
}
