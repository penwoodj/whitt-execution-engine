fn main() {
    use whitt_execution_engine::benchmark::generate_gpu_cpu_compare_benchmarks;
    use std::path::Path;

    let output_dir = Path::new("/home/jon/code/whitt-execution-engine/docs/workflows/benchmarks");
    println!("Generating benchmark YAML files to {}", output_dir.display());

    match generate_gpu_cpu_compare_benchmarks(output_dir) {
        Ok(()) => println!("Successfully generated all benchmark YAML files"),
        Err(e) => eprintln!("Error generating benchmarks: {:?}", e),
    }
}
