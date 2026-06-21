use ax_rnd::AxRng;
use std::fs;
use std::io::{self, BufRead, Write};

fn generate_uuid_v4(rnd: &mut AxRng) -> String {
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

            let mut rnd = AxRng::new(seed);
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

            let mut rnd = AxRng::new(seed);
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

            let mut rnd = AxRng::new(seed);
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

            let mut rnd = AxRng::new(seed);
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

            let mut rnd = AxRng::new(seed);
            println!("{}", rnd.token(len));
        }
        "shuffle" => {
            let mut seed: Option<u64> = None;
            let mut file_path: Option<String> = None;

            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "-f" | "--file" => {
                        if i + 1 < args.len() {
                            file_path = Some(args[i + 1].clone());
                            i += 2;
                        } else {
                            eprintln!("Error: -f/--file requires a path");
                            std::process::exit(1);
                        }
                    }
                    "-s" | "--seed" => {
                        if i + 1 < args.len() {
                            seed = parse_seed(&args[i + 1]);
                            if seed.is_none() {
                                eprintln!("Error: invalid seed value");
                                std::process::exit(1);
                            }
                            i += 2;
                        } else {
                            eprintln!("Error: -s/--seed requires a value");
                            std::process::exit(1);
                        }
                    }
                    _ => {
                        if file_path.is_none() && args[i].parse::<u64>().is_err() {
                            file_path = Some(args[i].clone());
                        } else if seed.is_none()
                            && let Some(s) = parse_seed(&args[i])
                        {
                            seed = Some(s);
                        }
                        i += 1;
                    }
                }
            }

            let seed = seed.unwrap_or_else(|| {
                use std::time::{SystemTime, UNIX_EPOCH};
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_else(|_| std::time::Duration::from_nanos(0xA0761D6478BD642F))
                    .as_nanos() as u64
            });

            let mut rnd = AxRng::new(seed);

            if let Some(path) = file_path {
                let content = fs::read_to_string(&path).unwrap_or_else(|e| {
                    eprintln!("Error: Cannot read file '{}': {}", path, e);
                    std::process::exit(1);
                });
                let mut lines: Vec<&str> = content.lines().collect();
                shuffle_lines(&mut lines, &mut rnd);
                for line in lines {
                    println!("{}", line);
                }
            } else {
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

fn shuffle_lines<T>(lines: &mut [T], rnd: &mut AxRng) {
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
    println!("AXRND - CLI tool for random number generation");
    println!();
    println!("USAGE:");
    println!("  ax_rnd <command> [args]");
    println!();
    println!("COMMANDS:");
    println!("  bytes [count] [--hex] [seed]  Generate random bytes");
    println!("  alpha [len]           [seed]  Generate alphanumeric (base62)");
    println!("  token [len]           [seed]  Generate URL-safe token (base64url)");
    println!("  shuffle [-f FILE] [-s SEED]   Shuffle lines from stdin or file");
    println!("  u64   [count]         [seed]  Generate random u64 numbers");
    println!("  uuid  [count]         [seed]  Generate UUID v4");
    println!();
    println!("OPTIONS:");
    println!("  count/len   Number of bytes/chars/numbers (default: 32)");
    println!("  --hex       Output bytes as hex string");
    println!("  seed        rng seed (default: time)");
    println!("              Use 'time' for current timestamp");
    println!();
    println!("EXAMPLES:");
    println!("  ax_rnd bytes 64");
    println!("  ax_rnd bytes 32 --hex");
    println!("  ax_rnd alpha 16");
    println!("  ax_rnd token 32");
    println!("  ax_rnd shuffle -f file.txt");
    println!("  ax_rnd shuffle -f file.txt -s 12345");
    println!("  echo 'a b c' | ax_rnd shuffle");
    println!("  echo 'a b c' | ax_rnd shuffle -s 42");
    println!("  ax_rnd u64 5");
    println!("  ax_rnd uuid 3");
    println!("  ax_rnd bytes 32 12345");
}
