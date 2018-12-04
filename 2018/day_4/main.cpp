#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <cstdio>
#include <map>

int main(int argc, char** argv) {
  std::ifstream infile("input.txt");
  std::string line;
  std::vector<std::string> logs;
  while (std::getline(infile, line)) {
    logs.emplace_back(line); 
  }
    std::sort(logs.begin(), logs.end());
  // for(const auto& item: logs) {
  //   std::cout << item << "\n";
  // }

  int year{0};
  int month{0};
  int day{0};
  int hour{0};
  int minute{0};
  int guard_id{0};
  std::string rest_of_the_line;
  std::map<int, int[60]> id_sleep_map;
  
  int start_of_the_nap{0};
  int end_of_the_nap{0};
  for(const auto& item: logs) {
    if (std::sscanf(item.c_str(), "[%d-%d-%d %d:%d]", &year, &month, &day, &hour, &minute) != 5) {
      std::cerr << "Improperly formatted line, can't parse[" << line << std::endl;
      continue;
    }
    rest_of_the_line = item.substr(item.find("]") + 2);
    if(rest_of_the_line.find("Guard") != std::string::npos) {
      std::sscanf(rest_of_the_line.c_str(), "Guard #%d begins shift", &guard_id);
    }
    else if(rest_of_the_line.find("wakes up") != std::string::npos) {
      end_of_the_nap = minute; 
      for(int i=start_of_the_nap; i < end_of_the_nap; i++) {
        id_sleep_map[guard_id][i]++; 
      }
    } else if(rest_of_the_line.find("falls asleep") != std::string::npos) {
      start_of_the_nap = minute;
    } else {
      std::cerr << "Unhandled rest of the line[" << rest_of_the_line << "]\n";
      continue;
    }
  }

  int guard_id_sleeping{0};
  int max_total_asleep{0};
  int total_asleep{0};
  for(const auto& item: id_sleep_map) {
    std::cout << item.first << ": ["; 
    for(int i = 0; i < 60; i++) {
      std::cout << item.second[i] << " ";
        total_asleep += item.second[i];
    }
    if(total_asleep > max_total_asleep) {
      max_total_asleep = total_asleep;
      guard_id_sleeping = item.first;
    }
    total_asleep = 0;
    std::cout << "]\n";
  }

  std::cout << "The most sleeping guard: " << guard_id_sleeping << " minute: " << max_total_asleep << "\n";
  int max_minutes{0};
  int max_sleeping_minute{0};
  for(int i=0; i<60; i++) {
    if(id_sleep_map[guard_id_sleeping][i] > max_minutes) {
      max_minutes = id_sleep_map[guard_id_sleeping][i];
      max_sleeping_minute = i;
    }
  }
  std::cout << "Guard: " << guard_id_sleeping << " sleeps the most at: " << max_sleeping_minute << "\n";
  std::cout << guard_id_sleeping * max_sleeping_minute << "\n";
  
  return 0;
}
