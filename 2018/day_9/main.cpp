#include <iostream>
#include <vector>
#include <list>
#include <algorithm>

//459 players; last marble is worth 71790 points

class Game { 
  public:
    Game(int players, int last_marble):
      m_players_no(players),
      m_last_marble(last_marble) {

      scores.resize(m_players_no);
      game_board.push_back(0);
    }

    void play_game() {
      auto current_iterator = game_board.cbegin();
      for(auto marble = 1, player = 0;
          marble != m_last_marble + 1;
          player = (player + 1 ) % m_players_no,
          marble++) {
        if(marble % 23) {
          if(++current_iterator == game_board.cend()) {
            current_iterator = game_board.cbegin();
          }
          current_iterator = game_board.insert(++current_iterator, marble);
        } else {
            for (auto i = 0; i != 7; ++i, --current_iterator) {
              if(current_iterator == game_board.cbegin()) {
                current_iterator = game_board.cend();
              }
            }
            scores[player] += marble + *current_iterator;
            game_board.erase(current_iterator++);
        }
      }
    }

    void print_scores() const {
      auto max_el = std::max_element(scores.begin(), scores.end());
      std::cout << "Highest score: " << *max_el << std::endl;
    }

  private: 
    int m_players_no;
    int m_last_marble;
    std::vector<long> scores;
    std::list<long> game_board; 
};

int main(int argc, char** argv) {
  constexpr int players_no = 459;
  constexpr int last_marble = 71790 * 100;

  Game g(players_no, last_marble);
  g.play_game();
  g.print_scores();

  return EXIT_SUCCESS;
}
