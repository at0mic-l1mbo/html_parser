use rand::Rng;
use reqwest;
use select::document::Document;
use select::node::Node;
use std::env;
use std::fs::OpenOptions;
use std::io::{self, Write};
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;

fn show_menu() {
    let message: &str = r#"
    ___ ___________________  .____      __________                                   
    /   |   \__    ___/     \ |    |     \______   \_____ _______  ______ ___________ 
   /    ~    \|    | /  \ /  \|    |      |     ___/\__  \\_  __ \/  ___// __ \_  __ \
   \    Y    /|    |/    Y    \    |___   |    |     / __ \|  | \/\___ \\  ___/|  | \/
    \___|_  / |____|\____|__  /_______ \  |____|    (____  /__|  /____  >\___  >__|   
          \/                \/        \/                 \/           \/     \/              
    "#;
    println!("{}", message);
}

fn get_random_ip() -> String {
    let mut rng = rand::thread_rng();
    let ip: String = (0..4)
        .map(|_| rng.gen_range(0..256).to_string())
        .collect::<Vec<String>>()
        .join(".");
    ip
}

fn download_html(url: &String) -> String {
    let random_ip = get_random_ip();
    use fake_useragent::UserAgents;
    let user_agents = UserAgents::new();
    let response = reqwest::blocking::Client::new()
        .get(url)
        .header("User-Agent", user_agents.random())
        .header("X-Forwarded-For", random_ip)
        .send();

    match response {
        Ok(res) => {
            if res.status().is_success() {
                let body = res.text().unwrap();
                println!("[+] Server answer");
                println!("Getting html... could take a minute!");
                return body;
            } else {
                eprintln!("[-] Error: {}", res.status().is_client_error());
                return String::new();
            }
        }
        Err(err) => {
            eprintln!("[-] Error: {:?}", err);
            println!("Remember to use http://www.website.com");
            return String::new();
        }
    }
}

fn clean_url(url: &str) -> String {
    let cleaned_url = url
        .splitn(2, '/')
        .next()
        .map_or(url, |s| s.trim_end_matches(|c| c == '/' || c == '?'));
    cleaned_url.to_string()
}

fn resolve_domain_ip(domain: &str) -> io::Result<Vec<String>> {
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    match resolver.lookup_ip(domain) {
        Ok(lookup) => {
            let ips: Vec<String> = lookup.iter().map(|ip| ip.to_string()).collect();
            Ok(ips)
        }
        Err(err) => Err(io::Error::new(io::ErrorKind::Other, format!("DNS resolution error: {:?}", err))),
    }
}

fn handle_html(html: String) -> io::Result<()> {
    if html.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Empty HTML!"));
    }

    let mut file = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open("index.txt")?;

    let document = Document::from_read(html.as_bytes())?;
    let href_filter = |node: &Node| {
        node.attr("href").map_or(false, |href| href.starts_with("h"))
    };

    for node in document.find(href_filter) {
        if let Some(href) = node.attr("href") {
            let cleaned_href = if href.starts_with("https://") {
                &href[8..]
            } else if href.starts_with("http://") {
                &href[7..]
            } else {
                href
            };
            let cleaned_url = clean_url(cleaned_href);

            match resolve_domain_ip(&cleaned_url) {
                Ok(ips) => {
                    for ip in ips {
                        writeln!(file, "{} {:?}", cleaned_url, ip)?;
                    }
                }
                Err(err) => {
                    eprintln!("[-] Error resolving IP for {}: {:?}", cleaned_url, err);
                }
            }
        }
    }

    println!("[+] Success operation, verify the index.txt!");
    Ok(())
}

fn main() {
    show_menu();
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("[-] Invalid arguments!");
        println!("Usage: cargo run -- https://www.google.com");
        return;
    }
    let website: &String = &args[1];
    let html: String = download_html(website);
    let _ = handle_html(html);
}
