use crate::{MapfBenchmarker, BenchmarkDescriptor, Problem, Coordinate, MarkedCoordinate};
use crate::solution::Solution;
use std::ffi::CStr;
use libc::c_char;

pub struct CallBack {
    cb: extern "C" fn(&Problem) -> *mut Solution
}

impl CallBack {
    pub fn call(&self, problem: Problem) -> Solution {
        let s = (self.cb)(&problem);

        if s == std::ptr::null_mut() {
            panic!("null solution")
        }

        let val = unsafe {
            Box::from_raw(s)
        };

        let res = *val.clone();

        drop(val);

        return res;
    }
}

unsafe fn get_str<'a>(inp: *const c_char) -> Option<&'a str> {
    let raw = CStr::from_ptr(inp);

    match raw.to_str() {
        Ok(s) => Some(s),
        Err(_) => None,
    }
}

unsafe fn create_benchmarker_helper(
    token: *const c_char,
    benchmark: BenchmarkDescriptor,
    algorithm_name: *const c_char,
    version: *const c_char,
    debug: bool,
    solver: CallBack,
    base_url: *const c_char,
) -> Option<MapfBenchmarker> {
    Some(MapfBenchmarker::new(
        get_str(token)?,
        [benchmark].to_vec(),
        get_str(algorithm_name)?,
        get_str(version)?,
        debug,
        solver,
        if base_url.is_null() {
            None
        } else {
            Some(get_str(base_url)?)
        }
    ))
}

#[cfg(feature = "cbindgen-on")]
#[no_mangle]
/// Create a new benchmarker (note: should be freed with free_benchmarker)
pub unsafe extern "C" fn create_benchmarker(
    token: *const c_char,
    benchmark: *mut BenchmarkDescriptor,
    algorithm_name: *const c_char,
    version: *const c_char,
    debug: bool,
    solver: extern "C" fn(&Problem) -> *mut Solution,
    base_url: *const c_char,
) -> *const MapfBenchmarker {
    if benchmark as *const _ == std::ptr::null() {
        return std::ptr::null()
    }

    if let Some(benchmark) = benchmark.as_ref() {

        if let Some(i) = create_benchmarker_helper(
            token,
            benchmark.clone(),
            algorithm_name,
            version,
            debug,
            CallBack {cb: solver},
            base_url
        ) {
            let res = Box::leak(Box::new(i));

            res as *const MapfBenchmarker
        } else {
            return std::ptr::null()
        }
    } else {
        return std::ptr::null()
    }
}

#[cfg(feature = "cbindgen-on")]
#[no_mangle]
/// Run the benchmark(s) on the benchmarker.
pub unsafe extern "C" fn run_benchmark(
    benchmarker: *const MapfBenchmarker,
    make_attempt: bool,
) {
    let benchmarker = Box::from_raw(benchmarker as *mut MapfBenchmarker);

    let res = if let Err(err) = benchmarker.run(make_attempt) {
        panic!("{:?}", err)
    };

    let _ = Box::leak(benchmarker);

    res
}

#[cfg(feature = "cbindgen-on")]
#[no_mangle]
/// Free a benchmarker.
pub unsafe extern "C" fn free_benchmarker(
    benchmarker: *mut MapfBenchmarker,
) {
    drop(Box::from_raw(benchmarker));
}


#[cfg(feature = "cbindgen-on")]
#[no_mangle]
/// Create a new benchmark descriptor from its benchmark id (from mapf.nl) (note: should be freed with free_benchmark_descriptor)
pub unsafe extern "C" fn create_benchmark_descriptor(
    number: usize
) -> *mut BenchmarkDescriptor {
    Box::leak(Box::new(BenchmarkDescriptor::from_identifier(number)))
}

#[cfg(feature = "cbindgen-on")]
#[no_mangle]
/// Free a benchmark descriptor
pub unsafe extern "C" fn free_benchmark_descriptor(
    descriptor: *mut BenchmarkDescriptor,
) {
    drop(Box::from_raw(descriptor));
}


#[cfg(feature = "cbindgen-on")]
#[no_mangle]
/// Allocates a new solution. Returning a solution from the solve function automatically frees the solution.
pub unsafe extern "C" fn create_solution() -> *mut Solution {
    Box::leak(Box::new(Solution{ paths: vec![] }))
}

pub struct Path {
    contents: Vec<Coordinate>
}

#[cfg(feature = "cbindgen-on")]
#[no_mangle]
/// Allocates a new path.
pub unsafe extern "C" fn create_path() -> *mut Path {
    Box::leak(Box::new(Path{ contents: vec![] }))
}

#[cfg(feature = "cbindgen-on")]
#[no_mangle]
/// Adds a coordinate to a path
pub unsafe extern "C" fn add_to_path(path: &mut Path, coord: Coordinate) {
    path.contents.push(coord);
}

#[cfg(feature = "cbindgen-on")]
#[no_mangle]
/// Add a path to a solution. Frees the path (do not use path after this)
pub unsafe extern "C" fn add_to_solution(solution: &mut Solution, path: &mut Path) {
    solution.paths.push(path.contents.clone());

    drop(Box::from_raw(path as *mut _));
}


#[cfg(feature = "cbindgen-on")]
#[no_mangle]
/// Get the width of a problem
pub unsafe extern "C" fn problem_width(problem: &Problem) -> usize {
    problem.grid.width()
}

#[cfg(feature = "cbindgen-on")]
#[no_mangle]
/// Get the height of a problem
pub unsafe extern "C" fn problem_height(problem: &Problem) -> usize {
    problem.grid.height()
}

#[cfg(feature = "cbindgen-on")]
#[no_mangle]
/// Find if there is a wall at the specified coordinate
pub unsafe extern "C" fn problem_wall_at(problem: &Problem, coordinate: Coordinate) -> bool {
    problem.grid.wall_at(coordinate).unwrap_or(false)
}

#[cfg(feature = "cbindgen-on")]
#[no_mangle]
/// Find if there is a wall at the specified coordinate
pub unsafe extern "C" fn problem_num_agents(problem: &Problem) -> usize {
    problem.starts.len()
}

#[cfg(feature = "cbindgen-on")]
#[no_mangle]
/// Find if there is a wall at the specified coordinate
pub unsafe extern "C" fn problem_agent_starts(problem: &Problem) -> *const MarkedCoordinate {
    problem.starts.as_ptr()
}

#[cfg(feature = "cbindgen-on")]
#[no_mangle]
/// Find if there is a wall at the specified coordinate
pub unsafe extern "C" fn problem_agent_goals(problem: &Problem) -> *const MarkedCoordinate {
    problem.goals.as_ptr()
}