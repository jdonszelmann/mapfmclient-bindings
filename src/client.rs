use std::error::Error;
use crate::solution::Solution;
use crate::problem::Problem;
use reqwest::blocking::Client;
use std::time::Duration;
use crate::marked::MarkedCoordinate;
use crate::coordinate::Coordinate;
use crate::grid::Grid;
use reqwest::Url;
use thiserror::Error;
use serde::{Serialize, Deserialize};
#[cfg(feature = "cbindgen-on")]
use crate::ffi::CallBack;

#[derive(Debug, Error)]
#[cfg_attr(feature = "cbindgen-on", repr(C))]
pub enum MapfmClientError {
    #[error("failed to parse url")]
    UrlParse(Box<dyn Error>),

    #[error("request error {0}")]
    RequestError(reqwest::Error),

    #[error("json decode error {0}")]
    JsonDecodeError(reqwest::Error),

    #[error("status code: {0}")]
    Status(u16)
}


#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "cbindgen-on", repr(C))]
pub struct ProgressiveDescriptor {
    max_agents: usize,
    min_agents: usize,
    num_teams: usize,
    max_diff: usize,
}

impl ProgressiveDescriptor {
    pub fn new(
        max_agents: usize,
        min_agents: usize,
        num_teams: usize,
        max_diff: usize,
    ) -> Self {
        Self {
            max_agents,
            min_agents,
            num_teams,
            max_diff
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "cbindgen-on", repr(C))]
pub struct BenchmarkDescriptor {
    identifier: usize,
    progressive_descriptor: Option<Box<ProgressiveDescriptor>>,
}

impl BenchmarkDescriptor {
    pub(crate) fn progressive(&self) -> bool {
        self.progressive_descriptor.is_some()
    }
}

impl BenchmarkDescriptor {
    pub fn new(identifier: usize, progressive_descriptor: Option<ProgressiveDescriptor>) -> Self {
        Self {
            identifier,
            progressive_descriptor: progressive_descriptor.map(Box::new)
        }
    }

    pub fn from_identifier(identifier: usize) -> Self {
        Self::new(identifier, None)
    }
}

#[cfg_attr(feature = "cbindgen-on", repr(C))]
pub struct MapfBenchmarker {
    token: String,
    benchmark_descriptors: Vec<BenchmarkDescriptor>,
    algorithm_name: String,
    version: String,
    debug: bool,

    #[cfg(feature = "cbindgen-on")]
    solver: CallBack,
    #[cfg(not(feature = "cbindgen-on"))]
    solver: fn(Problem) -> Solution,

    base_url: String,

    client: Client
}

impl MapfBenchmarker {
    #[cfg(feature = "cbindgen-on")]
    pub fn new(
        token: &str,
        benchmark: Vec<BenchmarkDescriptor>,
        algorithm_name: &str,
        version: &str,
        debug: bool,
        solver: CallBack,
        base_url: Option<&str>,
    ) -> Self {
        Self {
            token: token.to_string(),
            benchmark_descriptors: benchmark,
            algorithm_name: algorithm_name.to_string(),
            version: version.to_string(),
            debug,
            solver,
            base_url: base_url.map(|i| i.to_string()).unwrap_or("https://mapf.nl/".to_string()),

            client: Client::new()
        }
    }

    #[cfg(not(feature = "cbindgen-on"))]
    pub fn new(
        token: &str,
        benchmark: Vec<BenchmarkDescriptor>,
        algorithm_name: &str,
        version: &str,
        debug: bool,
        solver: fn(Problem) -> Solution,
        base_url: Option<&str>,
    ) -> Self {
        Self {
            token: token.to_string(),
            benchmark_descriptors: benchmark,
            algorithm_name: algorithm_name.to_string(),
            version: version.to_string(),
            debug,
            solver,
            base_url: base_url.map(|i| i.to_string()).unwrap_or("https://mapf.nl/".to_string()),

            client: Client::new()
        }
    }


    fn get_benchmark_data(&self, descriptor: &BenchmarkDescriptor, attempt: bool) -> GetBenchmarkData {
        GetBenchmarkData {
            algorithm: self.algorithm_name.clone(),
            version: self.version.clone(),
            debug: self.debug,
            progressive: descriptor.progressive(),
            progressive_description: descriptor.progressive_descriptor.clone(),
            create_attempt: attempt,
        }
    }

    fn get_benchmark(&self, descriptor: &BenchmarkDescriptor) -> Result<Vec<Problem>, MapfmClientError> {
        let url = Url::parse(&self.base_url)
            .map_err(|i| MapfmClientError::UrlParse(Box::new(i)))?
            .join(&format!("/api/benchmark/attempt/{}", descriptor.identifier))
            .map_err(|i| MapfmClientError::UrlParse(Box::new(i)))?;

        let req = self.client
            .post(url)
            .header("X-API-Token", &self.token)
            .json(&self.get_benchmark_data(descriptor, false));

        let resp = req.send().map_err(MapfmClientError::RequestError)?;


        let data: SerializedProblemList = resp.json().map_err(MapfmClientError::JsonDecodeError)?;

        Ok(data.benchmarks.iter().map(|i| i.to_problem()).collect())
    }

    fn start_attempt(&self, descriptor: &BenchmarkDescriptor) -> Result<(Vec<Problem>, i64), MapfmClientError> {
        let url = Url::parse(&self.base_url)
            .map_err(|i| MapfmClientError::UrlParse(Box::new(i)))?
            .join(&format!("/api/benchmark/attempt/{}", descriptor.identifier))
            .map_err(|i| MapfmClientError::UrlParse(Box::new(i)))?;

        let req = self.client
            .post(url)
            .header("X-API-Token", &self.token)
            .json(&self.get_benchmark_data(descriptor, true));

        let resp = req.send().map_err(MapfmClientError::RequestError)?;

        let status = resp.status().as_u16();
        if status != 200 {
            return Err(MapfmClientError::Status(status))
        }

        let data: SerializedProblemList = resp.json().map_err(MapfmClientError::JsonDecodeError)?;

        Ok((
            data.benchmarks.iter().map(|i| i.to_problem()).collect(),
            data.attempt_id
        ))
    }

    fn run_benchmark(&self, problems: Vec<Problem>) -> Vec<(Solution, Problem, Duration)> {

        problems.into_iter().map(|i|  {
            let t1 = std::time::Instant::now();

            #[cfg(feature = "cbindgen-on")]
            let solution = self.solver.call(i.clone());
            #[cfg(not(feature = "cbindgen-on"))]
            let solution = (self.solver)(i.clone());

            let duration = std::time::Instant::now().duration_since(t1);
            (solution, i, duration)
        }).collect()
    }

    fn submit_solutions(&self, descriptor: &BenchmarkDescriptor, solutions: Vec<(Solution, Problem, Duration)>, id: i64) -> Result<(), MapfmClientError> {
        let url = Url::parse(&self.base_url)
            .map_err(|i| MapfmClientError::UrlParse(Box::new(i)))?
            .join(&format!("/api/solutions/submit/{}", id))
            .map_err(|i| MapfmClientError::UrlParse(Box::new(i)))?;

        let req = self.client
            .post(url)
            .header("X-API-Token", &self.token)
            .json(&SubmitSolutionData {
                solutions: solutions.into_iter().map(|(sol, prob, time)| {
                    SerializedSolution {
                        time: time.as_nanos(),
                        solution: SerializedSolutionData {
                            paths: sol.paths.into_iter()
                                .map(|i| SerializedPath {
                                    route: i
                                })
                                .collect()
                        },
                        progressive_params: descriptor.progressive_descriptor.clone().map(|i| {
                            ProgressiveParams {
                                num_agents: prob.starts.len(),
                                num_teams: i.num_teams,
                                max_diff: i.max_diff,
                                starts: prob.starts,
                                goals: prob.goals
                            }
                        })
                    }
                }).collect(),
                benchmark: descriptor.identifier,
                progressive: descriptor.progressive()
            });


        let resp = req.send().map_err(MapfmClientError::RequestError)?;
        let status = resp.status().as_u16();
        if status != 200 {
            return Err(MapfmClientError::Status(status))
        }

        Ok(())
    }

    pub fn run(&self, make_attempt: bool) -> Result<(), MapfmClientError> {
        for descriptor in &self.benchmark_descriptors {
            if make_attempt {
                let (problems, id) = self.start_attempt(descriptor)?;

                let solutions = self.run_benchmark(problems);
                self.submit_solutions(descriptor, solutions, id)?;
            } else {
                let problems = self.get_benchmark(descriptor)?;

                self.run_benchmark(problems);
            }
        }

        Ok(())
    }
}

#[derive(Serialize)]
struct ProgressiveParams {
    num_agents: usize,
    num_teams: usize,
    max_diff: usize,

    starts: Vec<MarkedCoordinate>,
    goals: Vec<MarkedCoordinate>,
}

#[derive(Serialize)]
struct SerializedPath {
    route: Vec<Coordinate>
}

#[derive(Serialize)]
struct SerializedSolutionData {
    paths: Vec<SerializedPath>
}

#[derive(Serialize)]
struct SerializedSolution {
    time: u128,
    solution: SerializedSolutionData,
    progressive_params: Option<ProgressiveParams>
}

#[derive(Serialize)]
struct SubmitSolutionData {
    solutions: Vec<SerializedSolution>,
    benchmark: usize,
    progressive: bool,
}

#[derive(Deserialize)]
struct SerializedProblemList {
    benchmarks: Vec<SerializedProblem>,

    #[serde(default)]
    attempt_id: i64,
}

#[derive(Deserialize)]
struct SerializedProblem {
    width: usize,
    height: usize,
    grid: Vec<Vec<i64>>,
    starts: Vec<MarkedCoordinate>,
    goals: Vec<MarkedCoordinate>,
}

impl SerializedProblem {
    pub fn to_problem(&self) -> Problem {
        Problem {
            grid: Grid::from_int_vecs(self.width, self.height, self.grid.clone()),
            starts: self.starts.clone(),
            goals: self.goals.clone(),
        }
    }
}

#[derive(Serialize)]
struct GetBenchmarkData {
    algorithm: String,
    version: String,
    debug: bool,
    progressive: bool,
    progressive_description: Option<Box<ProgressiveDescriptor>>,
    create_attempt: bool,
}


#[cfg(test)]
mod tests {
    use crate::client::{BenchmarkDescriptor, MapfBenchmarker};
    use crate::problem::Problem;
    use crate::solution::Solution;
    use crate::coordinate::Coordinate;

    #[test]
    #[cfg(not(feature = "cbindgen-on"))]
    pub fn test() {
        let benchmark = BenchmarkDescriptor::from_identifier(1);
        let token = "Y2S9hyWDTbHC7cNl3kllKb3JB0EK";

        fn test (problem: Problem) -> Solution {
            dbg!(problem);

            return Solution {
                paths: vec![
                    vec![
                        Coordinate::new(2, 1),
                        Coordinate::new(1, 1),
                        Coordinate::new(0, 1),
                    ]
                ]
            }
        }

        let bm = MapfBenchmarker::new(
            token,
            vec![benchmark],
            "test",
            "test",
            true,
            test,
            None,
        );

        bm.run(true).unwrap();
    }
}

