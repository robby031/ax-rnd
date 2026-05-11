use ax_rnd::Axrnd;
use std::fs;
use std::io::{self, BufRead, Write};

fn generate_uuid_v4(rnd: &mut Axrnd) -> String {
    let mut bytes = [0u8; 16];
    rnd.fill_bytes(&mut bytes);
    bytes[6] = (bytes[6] & 0x0F) | 0x40;
    bytes[8] = (bytes[8] & 0x3F) | 0x80;

    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0],
        bytes[1],
        bytes[2],
        bytes[3],
        bytes[4],
        bytes[5],
        bytes[6],
        bytes[7],
        bytes[8],
        bytes[9],
        bytes[10],
        bytes[11],
        bytes[12],
        bytes[13],
        bytes[14],
        bytes[15]
    )
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "bytes" => {
            let count = args
                .get(2)
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(32);
            let hex = args.get(3).map(|s| s == "--hex").unwrap_or(false);
            let seed = args.get(4).and_then(|s| parse_seed(s)).unwrap_or_else(|| {
                use std::time::{SystemTime, UNIX_EPOCH};
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64
            });

            let mut rnd = Axrnd::new(seed);
            let mut buf = vec![0u8; count];
            rnd.fill_bytes(&mut buf);

            if hex {
                for b in buf {
                    print!("{:02x}", b);
                }
                println!();
            } else {
                io::stdout().write_all(&buf).unwrap();
            }
        }
        "alpha" => {
            let len = args
                .get(2)
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(32);
            let seed = args.get(3).and_then(|s| parse_seed(s)).unwrap_or_else(|| {
                use std::time::{SystemTime, UNIX_EPOCH};
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64
            });

            let mut rnd = Axrnd::new(seed);
            println!("{}", rnd.alpha(len));
        }
        "u64" => {
            let count = args
                .get(2)
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(1);
            let seed = args.get(3).and_then(|s| parse_seed(s)).unwrap_or_else(|| {
                use std::time::{SystemTime, UNIX_EPOCH};
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64
            });

            let mut rnd = Axrnd::new(seed);
            for _ in 0..count {
                println!("{}", rnd.next_u64());
            }
        }
        "uuid" => {
            let count = args
                .get(2)
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(1);
            let seed = args.get(3).and_then(|s| parse_seed(s)).unwrap_or_else(|| {
                use std::time::{SystemTime, UNIX_EPOCH};
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64
            });

            let mut rnd = Axrnd::new(seed);
            for _ in 0..count {
                println!("{}", generate_uuid_v4(&mut rnd));
            }
        }
        "token" => {
            let len = args
                .get(2)
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(32);
            let seed = args.get(3).and_then(|s| parse_seed(s)).unwrap_or_else(|| {
                use std::time::{SystemTime, UNIX_EPOCH};
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64
            });

            let mut rnd = Axrnd::new(seed);
            println!("{}", rnd.token(len));
        }
        "shuffle" => {
            let seed = args.get(2).and_then(|s| parse_seed(s)).unwrap_or_else(|| {
                use std::time::{SystemTime, UNIX_EPOCH};
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64
            });

            let mut rnd = Axrnd::new(seed);

            if let Some(file_path) = args.get(2) {
                if file_path.parse::<u64>().is_err() {
                    let content = fs::read_to_string(file_path).unwrap_or_else(|_| {
                        eprintln!("Error: Cannot read file: {}", file_path);
                        std::process::exit(1);
                    });
                    let mut lines: Vec<&str> = content.lines().collect();
                    shuffle_lines(&mut lines, &mut rnd);
                    for line in lines {
                        println!("{}", line);
                    }
                    return;
                }
            }

            let stdin = io::stdin();
            let mut lines: Vec<String> = Vec::new();
            for line in stdin.lock().lines() {
                match line {
                    Ok(l) => lines.push(l),
                    Err(_) => break,
                }
            }
            shuffle_lines(&mut lines, &mut rnd);
            for line in lines {
                println!("{}", line);
            }
        }
        _ => print_usage(),
    }
}

fn parse_seed(s: &str) -> Option<u64> {
    if s == "time" {
        use std::time::{SystemTime, UNIX_EPOCH};
        Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
        )
    } else {
        s.parse::<u64>().ok()
    }
}

fn shuffle_lines<T>(lines: &mut [T], rnd: &mut Axrnd) {
    let len = lines.len();
    if len <= 1 {
        return;
    }
    for i in (1..len).rev() {
        let j = rnd.bounded_u64(i as u64) as usize;
        lines.swap(i, j);
    }
}

fn print_usage() {
    println!("axrnd - Fast rnd CLI");
    println!();
    println!("USAGE:");
    println!("  axrnd <command> [args]");
    println!();
    println!("COMMANDS:");
    println!("  bytes [count] [--hex] [seed]  Generate random bytes");
    println!("  alpha [len]           [seed]  Generate alphanumeric (base62)");
    println!("  token [len]           [seed]  Generate URL-safe token (base64url)");
    println!("  shuffle [file]        [seed]  Shuffle lines from stdin or file");
    println!("  u64   [count]         [seed]  Generate random u64 numbers");
    println!("  uuid  [count]         [seed]  Generate UUID v4");
    println!();
    println!("OPTIONS:");
    println!("  count/len   Number of bytes/chars/numbers (default: 32)");
    println!("  --hex       Output bytes as hex string");
    println!("  seed        rnd seed (default: time)");
    println!("              Use 'time' for current timestamp");
    println!();
    println!("EXAMPLES:");
    println!("  axrnd bytes 64");
    println!("  axrnd bytes 32 --hex");
    println!("  axrnd alpha 16");
    println!("  axrnd token 32");
    println!("  axrnd shuffle file.txt");
    println!("  echo 'a b c' | axrnd shuffle");
    println!("  axrnd u64 5");
    println!("  axrnd uuid 3");
    println!("  axrnd bytes 32 12345");
}
