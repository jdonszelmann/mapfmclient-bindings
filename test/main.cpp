
#include "bindings.h"
#include <iostream>

using namespace std;

Solution *solve(const Problem *problem) {
  cout << "width " << problem_width(problem) << endl;
  cout << "height " << problem_height(problem) << endl;
  for (int i = 0; i < problem_width(problem); ++i) {
    for (int j = 0; j < problem_height(problem); ++j) {
      if (problem_wall_at(problem, Coordinate{j, i})) {
        cout << "#";
      } else {
        cout << " ";
      }
    }
    cout << endl;
  }

  unsigned long num_agents = problem_num_agents(problem);
  cout << "num agents " << num_agents << endl;
  const MarkedCoordinate * starts = problem_agent_starts(problem);
  for (int i = 0; i < num_agents; i++) {
    auto start = starts[i];
    cout<< "agent start at " << start.coord.x << ", " << start.coord.y << " with colour " << start.colour << endl;
  }

  const MarkedCoordinate * goals = problem_agent_goals(problem);
  for (int i = 0; i < num_agents; i++) {
    auto start = goals[i];
    cout<< "agent goals at " << start.coord.x << ", " << start.coord.y << " with colour " << start.colour << endl;
  }


  // create solution
  auto solution = create_solution();

  auto path = create_path();
  add_to_path(path, Coordinate{2, 1});
  add_to_path(path, Coordinate{1, 1});
  add_to_path(path, Coordinate{0, 1});

  add_to_solution(solution, path);

  return solution;
}

int main(int argc, const char *argv[]) {
  const char *token = "Y2S9hyWDTbHC7cNl3kllKb3JB0EK";

  auto bmd = create_benchmark_descriptor(1);

  auto bm =
      create_benchmarker(token, bmd, "test", "version 0", true, solve, nullptr);

  run_benchmark(bm, true);

  free_benchmark_descriptor(bmd);
}