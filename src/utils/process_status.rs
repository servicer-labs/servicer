use tokio::{
    fs::File,
    io::{AsyncBufReadExt, AsyncReadExt, BufReader},
};

/// Gets the kernel page size of the system in KB
pub async fn get_page_size() -> usize {
    let path = "/proc/self/smaps";
    let file = File::open(path).await.unwrap();
    let reader = BufReader::new(file);

    let mut kernel_page_size: Option<usize> = None;

    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await.unwrap() {
        if line.starts_with("KernelPageSize:") {
            if let Some(size_str) = line.split_whitespace().nth(1) {
                if let Ok(size) = size_str.parse::<usize>() {
                    kernel_page_size = Some(size);
                    break;
                }
            }
        }
    }

    kernel_page_size.unwrap()
}

/// Gets the memory used by a process in KB
///
/// Formula from QPS: (rss pages - shared pages) * page size
///
/// # Arguments
///
/// * `pid` - Process ID
/// * `page_size` - The page size in KB
///
pub async fn get_memory_usage(pid: u32, page_size_kb: u64) -> u64 {
    let path = format!("/proc/{}/statm", pid);
    let mut file = File::open(&path).await.unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.unwrap();

    let values: Vec<&str> = contents.trim().split_whitespace().collect();
    if values.len() < 2 {
        panic!("Invalid format of /proc/PID/statm file");
    }

    let rss_pages: u64 = values[1].parse().unwrap_or(0);
    let shared_pages: u64 = values[2].parse().unwrap_or(0);

    (rss_pages - shared_pages) * page_size_kb
}
