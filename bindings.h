#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct BenchmarkDescriptor;

struct MapfBenchmarker;

struct Path;

struct Problem;

struct Solution;

struct Coordinate {
  int64_t x;
  int64_t y;
};

struct MarkedCoordinate {
  Coordinate coord;
  int64_t colour;
};

extern "C" {

/// Create a new benchmarker (note: should be freed with free_benchmarker)
const MapfBenchmarker *create_benchmarker(const char *token,
                                          BenchmarkDescriptor *benchmark,
                                          const char *algorithm_name,
                                          const char *version,
                                          bool debug,
                                          Solution *(*solver)(const Problem*),
                                          const char *base_url);

/// Run the benchmark(s) on the benchmarker.
void run_benchmark(const MapfBenchmarker *benchmarker, bool make_attempt);

/// Free a benchmarker.
void free_benchmarker(MapfBenchmarker *benchmarker);

/// Create a new benchmark descriptor from its benchmark id (from mapf.nl) (note: should be freed with free_benchmark_descriptor)
BenchmarkDescriptor *create_benchmark_descriptor(uintptr_t number);

/// Free a benchmark descriptor
void free_benchmark_descriptor(BenchmarkDescriptor *descriptor);

/// Allocates a new solution. Returning a solution from the solve function automatically frees the solution.
Solution *create_solution();

/// Allocates a new path.
Path *create_path();

/// Adds a coordinate to a path
void add_to_path(Path *path, Coordinate coord);

/// Add a path to a solution. Frees the path (do not use path after this)
void add_to_solution(Solution *solution, Path *path);

/// Get the width of a problem
uintptr_t problem_width(const Problem *problem);

/// Get the height of a problem
uintptr_t problem_height(const Problem *problem);

/// Find if there is a wall at the specified coordinate
bool problem_wall_at(const Problem *problem, Coordinate coordinate);

/// Find if there is a wall at the specified coordinate
uintptr_t problem_num_agents(const Problem *problem);

/// Find if there is a wall at the specified coordinate
const MarkedCoordinate *problem_agent_starts(const Problem *problem);

/// Find if there is a wall at the specified coordinate
const MarkedCoordinate *problem_agent_goals(const Problem *problem);

} // extern "C"
