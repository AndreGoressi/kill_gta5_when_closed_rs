// Use this to prevent the console from appearing
// #![windows_subsystem = "windows"]
#[cfg(feature = "clap")]
extern crate clap;
extern crate winapi;

mod processes;
mod raw;
use processes::get_processes_by_name;
use std::{thread, time};

#[cfg(feature = "clap")]
use clap::Clap;

const DEFAULT_PROCESS_NAME: &str = "gta5_enhanced.exe";

#[cfg(feature = "clap")]
#[derive(Clap)]
#[clap(version = "1.0", author = "Santiago Saavedra")]
struct ClapCliOpts {
    #[clap(short, long, env, default_value = DEFAULT_PROCESS_NAME)]
    process_name: String,

    #[clap(short, parse(from_occurrences))]
    verbose: i32,

    #[clap(short = 'n', long)]
    dry_run: bool,

    #[clap(long)]
    loop_seconds: Option<u64>,
}

#[cfg(not(feature = "clap"))]
struct BaseCliOpts {
    process_name: String,
    verbose: i32,
    dry_run: bool,
    loop_seconds: Option<u64>,
}

trait OptionFlatmap<T> {
    type Item;
    fn flat_map<U, F, I>(self, f: F) -> Option<U>
    where
        Self: Sized,
        F: FnOnce(T) -> I,
        I: IntoIterator<Item = U>;
}

impl<T> OptionFlatmap<T> for Option<T> {
    type Item = T;
    fn flat_map<U, F, I>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> I,
        I: IntoIterator<Item = U>,
    {
        self.map(|item| f(item).into_iter().next()).unwrap_or(None)
    }
}

fn main() {
    #[cfg(feature = "clap")]
    let opts = ClapCliOpts::parse();
    #[cfg(not(feature = "clap"))]
    let opts = BaseCliOpts {
        process_name: DEFAULT_PROCESS_NAME.to_string(),
        dry_run: false,
        loop_seconds: Some(1),
        verbose: 1,
    };

    if opts.verbose > 0 {
        println!("Running in verbose mode.");
    }

    loop {
        let mut inside_loop = false;
        get_processes_by_name(opts.process_name.as_str(), Some(1))
            .into_iter()
            .for_each(|item| {
                inside_loop = true;
                let item_window = item.get_main_window();

                let (pid, item_name, item_window_title) = (
                    item.pid,
                    item.name.clone(),
                    item_window
                        .flat_map(|window: processes::Window| window.title().ok())
                        .unwrap_or(String::new()),
                );

                if item_window.is_some() {
                    if opts.verbose > 0 {
                        println!("ITEM: {} ({}), has_window? true, title: {}", item_name, pid, item_window_title);
                    }
                } 
                else {
                    if opts.verbose > 0 && !opts.dry_run {
                        println!("Killing the process {}", pid);
                    } 
                    else if opts.verbose > 0 && opts.dry_run {
                        println!("[DRY RUN] Would kill the process {}", pid);
                    }
                        if !opts.dry_run {
                        item.kill(None).ok();
                    }
                }

                if opts.verbose > 0 {
                    println!(
                        "ITEM: {} ({}), has_window? {}, title: {}",
                        item_name,
                        pid,
                        item_window.is_some(),
                        item_window_title
                    );
                }
            });

        let default_seconds = if opts.verbose > 0 { 1 } else { 120 };

        thread::sleep(time::Duration::from_secs(
            opts.loop_seconds.unwrap_or(default_seconds),
        ));
    }
}
