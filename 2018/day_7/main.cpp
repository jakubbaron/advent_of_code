#include <iostream>
#include <fstream>
#include <memory>
#include <vector>
#include <map>
#include <queue>

class Task;
using STask = std::shared_ptr<Task>;

class Task {
  public:
    Task(const std::string& id):
      m_id(id),
      m_is_worked_on(false),
      m_added_to_queue(false),
      m_time_to_finish(60 + (id[0] - 'A' + 1)),
      m_progress(0) {
    }

    void add_prerequisitive(STask stask) {
      m_prerquisitves.push_back(stask);
    }

    const auto is_finished() const {
      return (m_progress == m_time_to_finish);
    }

    const auto get_id() const {
      return m_id;
    }

    auto can_be_started() const {
      for(const auto& item: m_prerquisitves) {
        if(!item->is_finished()) {
          return false;
        }
      }
      return true;
    }
    void increment_progress() {
      if(m_progress < m_time_to_finish) {
        m_progress++;
      }
    }
    
    void set_added_to_queue() {
      m_added_to_queue = true;
    }
    bool is_added_to_queue() {
      return m_added_to_queue;
    }

    void print_prerequisitives() const {
      std::cout << "Item's [" << m_id << "] prerequisitves: ";
      for(const auto& item: m_prerquisitves) {
        std::cout << item->get_id() << " ";
      }
      std::cout << std::endl;
    }
    auto is_worked_on() const {
      return m_is_worked_on;
    }
    void set_is_worked_on(bool is_worked_on) {
      m_is_worked_on = is_worked_on;
    }
    auto get_progress() const {
      return "(" + std::to_string(m_progress) + "/" + std::to_string(m_time_to_finish) + ")";
    }

  private:
    std::string m_id;
    bool m_is_worked_on;
    bool m_added_to_queue;
    int m_time_to_finish;
    int m_progress;
    std::vector<STask> m_prerquisitves;
};

class Elf {
  public:
    Elf(): m_stask(nullptr) {
    }

    void assign_task(STask stask) {
      m_stask = stask;
      m_stask->set_is_worked_on(true);
    }

    void increment_work() {
      if(m_stask == nullptr) {
        return;
      }

      m_stask->increment_progress();
      if(m_stask->is_finished()) {
        m_stask->set_is_worked_on(false);
        m_stask = nullptr;
      }
    }

    bool is_free() const {
      return m_stask == nullptr;
    }
    auto working_on() const {
      if(m_stask == nullptr) {
        std::string progress = ".: (00/00)";
        return progress;
      }
      std::string progress = m_stask->get_id() + ": " + m_stask->get_progress();
      return progress;
    }
  private:
    STask m_stask;
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
  std::vector<Elf> elfs{5,Elf{}};

  bool all_finished = false;
  int full_time{0};
  std::queue<STask> task_queue;
  std::cout << "Time\tElf 1\tElf 2\t Elf 3\t Elf 4\t Elf 5\t Done\t\n";
  while(!all_finished) {
    all_finished = true;
    std::string done{};
    for(const auto& item: map_id) {
      const auto& stask = item.second;
      if(stask->can_be_started() && !stask->is_worked_on() && !stask->is_added_to_queue()) {
        std::cout << "Pushing task: " << stask->get_id() << " on queue\n";
        task_queue.push(stask);
        stask->set_added_to_queue();
      }
      if(stask->is_finished()) {
        done += stask->get_id();
      }
      all_finished &= stask->is_finished();
    }
    for(auto& elf: elfs) {
      if(task_queue.empty()) {
        break;
      }
      if(elf.is_free()) {
        auto stask = task_queue.front();
        elf.assign_task(stask);
        task_queue.pop();
      }
    }

    std::cout << std::to_string(full_time) << "\t";
    for(auto& elf: elfs) {
      std::cout <<elf.working_on() <<  "\t";
    }
    std::cout << done << std::endl;

    for(auto& elf: elfs) {
      elf.increment_work();
    }
    if(!all_finished) {
      full_time++;
    }
  }
  std::cout << "Total time taken: " << full_time << std::endl;

  return EXIT_SUCCESS;
}
