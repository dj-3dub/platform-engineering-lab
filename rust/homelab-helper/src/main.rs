use clap::{Parser, Subcommand};
use std::net::{TcpStream, ToSocketAddrs};
use std::process::Command;
use std::time::Duration;

#[derive(Parser)]
#[command(author, version, about = "Homelab health checker CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// ICMP ping a host using the system ping command
    Ping {
        /// Hostname or IP
        host: String,

        /// Number of echo requests
        #[arg(short, long, default_value_t = 1)]
        count: u8,
    },

    /// Check if a TCP port is open
    Port {
        /// Hostname or IP
        host: String,

        /// TCP port
        port: u16,

        /// Timeout (ms)
        #[arg(short, long, default_value_t = 2000)]
        timeout_ms: u64,
    },

    /// Simple HTTP / HTTPS health check
    Http {
        /// URL like http://192.168.2.51
        url: String,

        /// Timeout (ms)
        #[arg(short, long, default_value_t = 2000)]
        timeout_ms: u64,
    },

    /// DNS lookup
    Dns {
        /// Hostname to resolve
        host: String,
    },

    /// Scan a range of ports
    Scan {
        /// Hostname or IP
        host: String,

        /// Start port
        start: u16,

        /// End port
        end: u16,

        /// Timeout per port (ms)
        #[arg(short, long, default_value_t = 500)]
        timeout_ms: u64,
    },

    /// Full check bundle (DNS + ping + ports + HTTP)
    Check {
        /// Hostname or IP
        host: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ping { host, count } => ping_host(&host, count),
        Commands::Port {
            host,
            port,
            timeout_ms,
        } => check_port(&host, port, timeout_ms),
        Commands::Http { url, timeout_ms } => http_check(&url, timeout_ms),
        Commands::Dns { host } => dns_lookup_cmd(&host),
        Commands::Scan {
            host,
            start,
            end,
            timeout_ms,
        } => port_scan(&host, start, end, timeout_ms),
        Commands::Check { host } => full_check(&host),
    }
}

//
// -----------------------------
//  DNS Lookup
// -----------------------------
//

fn dns_lookup_cmd(host: &str) {
    println!("Resolving hostname: {}", host);

    match (host, 0).to_socket_addrs() {
        Ok(lookup) => {
            for addr in lookup {
                println!("  → {}", addr);
            }
        }
        Err(e) => {
            println!("❌ DNS lookup failed: {}", e);
        }
    }
}

//
// -----------------------------
//  ICMP Ping
// -----------------------------
//

fn ping_host(host: &str, count: u8) {
    println!("Pinging {} ({} packets)...", host, count);

    let output = Command::new("ping")
        .arg("-c")
        .arg(count.to_string())
        .arg(host)
        .output();

    match output {
        Ok(o) => {
            println!("{}", String::from_utf8_lossy(&o.stdout));
            if o.status.success() {
                println!("✔️ Ping successful");
            } else {
                println!("❌ Ping failed");
            }
        }
        Err(e) => println!("❌ Failed to execute ping: {}", e),
    }
}

//
// -----------------------------
//  TCP Port Check
// -----------------------------
//

fn check_port(host: &str, port: u16, timeout_ms: u64) {
    let addr = format!("{}:{}", host, port);
    let timeout = Duration::from_millis(timeout_ms);

    print!("Checking TCP port {} ... ", addr);

    match TcpStream::connect_timeout(&addr.parse().unwrap(), timeout) {
        Ok(_) => println!("OPEN"),
        Err(_) => println!("CLOSED"),
    }
}

//
// -----------------------------
//  HTTP Check
// -----------------------------
//

fn http_check(url: &str, timeout_ms: u64) {
    println!("Checking HTTP: {}", url);

    let output = Command::new("curl")
        .arg("-I")
        .arg("--max-time")
        .arg((timeout_ms / 1000).to_string())
        .arg(url)
        .output();

    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            let status_line = stdout.lines().next().unwrap_or("NO RESPONSE");

            println!("  → {}", status_line);

            if status_line.contains("200 OK") {
                println!("✔️ HTTP OK");
            } else {
                println!("❌ HTTP check failed");
            }
        }
        Err(e) => println!("❌ Failed to perform HTTP check: {}", e),
    }
}

//
// -----------------------------
//  Port Scan
// -----------------------------
//

fn port_scan(host: &str, start: u16, end: u16, timeout_ms: u64) {
    println!("Scanning ports {}–{} on {} ...", start, end, host);

    for port in start..=end {
        check_port(host, port, timeout_ms);
    }
}

//
// -----------------------------
//  Full Check Bundle
// -----------------------------
//

fn full_check(host: &str) {
    println!("============================================");
    println!("      FULL HEALTH CHECK FOR: {}", host);
    println!("============================================");

    println!("\n[1/4] DNS Lookup\n----------------");
    dns_lookup_cmd(host);

    println!("\n[2/4] Ping\n----------");
    ping_host(host, 3);

    println!("\n[3/4] Common Ports\n------------------");
    for port in [22, 80, 443] {
        check_port(host, port, 800);
    }

    println!("\n[4/4] HTTP / HTTPS Checks\n--------------------------");
    http_check(&format!("http://{}", host), 1500);
    http_check(&format!("https://{}", host), 1500);

    println!("\n============================================");
    println!("      FULL CHECK COMPLETE for {}", host);
    println!("============================================");
}
