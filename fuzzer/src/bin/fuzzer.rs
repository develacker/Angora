#[macro_use]
extern crate clap;
use clap::{App, Arg};

extern crate angora;
extern crate angora_common;
use angora::fuzz_main;

fn main() {
    let matches = App::new("angora-fuzzer")
        .version(crate_version!())
        .about("fuzz sth.")
        .arg(Arg::with_name("input_dir")
             .short("i")
             .long("input")
             .value_name("DIR")
             .help("Sets the directory of input seeds, use \"-\" to restart existed output direcotry")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("output_dir")
             .short("o")
             .long("output")
             .value_name("DIR")
             .help("Sets the directory of outputs")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("track_target")
             .short("t")
             .long("track")
             .value_name("PROM")
             .help("Sets the target for tracking, including taints, cmps.  Only set in LLVM mode.")
             .takes_value(true))
        .arg(Arg::with_name("pargs")
            .help("Targeted program and arguments")
            .required(true)
            .multiple(true)
            .allow_hyphen_values(true)
            .last(true)
            .index(1))
        .arg(Arg::with_name("memory_limit")
             .short("M")
             .long("memory_limit")
             .value_name("MEM")
             .help("Memory limit for programs, defualt is 200(MB)")
             .takes_value(true))
        .arg(Arg::with_name("time_limit")
             .short("T")
             .long("time_limit")
             .value_name("TIME")
             .help("time limit for programs, defualt is 1(s), the tracking timeout is 12 * TIME")
             .takes_value(true))
        .arg(Arg::with_name("thread_jobs")
             .short("j")
             .long("jobs")
             .value_name("JOB")
             .help("Sets the number of thread jobs, defualt is 1")
             .takes_value(true))
       .arg(Arg::with_name("search_method")
             .short("r")
             .long("search_method")
             .value_name("SearchMethod")
             .help("Which method to run the program in?")
             .possible_values(&["gd", "random", "mb"]))
        .arg(Arg::with_name("sync_afl")
             .short("S")
             .long("sync_afl")
             .help("sync the seeds with AFL. Output directory should be in AFL's directory strucutre."))
       .arg(Arg::with_name("disable_afl_mutation")
             .short("A")
             .long("disable_afl_mutaion")
             .help("disable the fuzzer to mutate inputs using AFL's mutation strategies"))
        .arg(Arg::with_name("disable_exploitation")
             .short("E")
             .long("disable_exploitation")
             .help("Disable the fuzzer to mutate sensititve bytes to exploit bugs"))
       .get_matches();

    fuzz_main(
        matches.value_of("input_dir").unwrap(),
        matches.value_of("output_dir").unwrap(),
        matches.value_of("track_target").unwrap_or("-"),
        matches.values_of_lossy("pargs").unwrap(),
        value_t!(matches, "thread_jobs", usize).unwrap_or(1),
        value_t!(matches, "memory_limit", u64).unwrap_or(angora_common::config::MEM_LIMIT),
        value_t!(matches, "time_limit", u64).unwrap_or(angora_common::config::TIME_LIMIT),
        matches.value_of("search_method").unwrap_or("gd"),
        matches.occurrences_of("sync_afl") > 0,
        matches.occurrences_of("disable_afl_mutation") == 0,
        matches.occurrences_of("disable_exploitation") == 0,
    );
}
