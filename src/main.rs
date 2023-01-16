use argh::FromArgs;
use std::process::Command;

#[doc = "A concise wrapper over both 'git clone' and 'yay -S'"]
#[derive(Debug, FromArgs)]
struct Fastener {
    /// clone from the AUR
    #[argh(switch, short = 'a')]
    aur: bool,
    /// the package to clone
    #[argh(positional, short = 'x')]
    package: String,
}

#[derive(Debug, PartialEq, Eq)]
enum FastenerErr {
    /// Failed fetching package
    FailedFetchingPackage,
    /// Fastener mode is 'GITHUB' and 'package' does not contain a slash
    NoSlash,
    /// Package parsing failed
    ParseErr,
}

#[derive(Debug, PartialEq, Eq)]
enum Mode {
    /// Clone from the AUR
    AUR,
    /// Clone from Github
    GITHUB(String),
}

impl Fastener {
    fn check(&self) -> Result<Mode, FastenerErr> {
        let _ = match self.package.contains('/') {
            false => match self.aur {
                true => return Ok(Mode::AUR),
                false => return Err(FastenerErr::NoSlash),
            },
            true => {
                if let Ok(package) = self.fmt_git() {
                    return Ok(Mode::GITHUB(package));
                } else {
                    return Err(FastenerErr::ParseErr);
                }
            }
        };
    }
    fn fmt_git(&self) -> Result<String, FastenerErr> {
        if let Some((host, repo)) = self.package.split_once('/') {
            return Ok(format!("https://github.com/{}/{}", host, repo));
        } else {
            return Err(FastenerErr::ParseErr);
        }
    }
    fn run(&self) -> Result<(), FastenerErr> {
        #[rustfmt::skip]
        let mode = match self.check() {
            Ok(mode) => { mode },
            Err(err) => { return Err(err); },
        };

        match mode {
            Mode::GITHUB(x) => {
                let params = vec!["clone", &x];
                let mut proc = Command::new("git")
                    .args(&params)
                    .spawn()
                    .expect("Failed to execute fastener process");
                let _ = proc
                    .wait()
                    .map_err(|_| return FastenerErr::FailedFetchingPackage);
                return Ok(());
            }
            Mode::AUR => {
                let params = vec!["-S", &self.package];
                let mut proc = Command::new("yay")
                    .args(&params)
                    .spawn()
                    .expect("Failed to execute fastener process");
                let _ = proc
                    .wait()
                    .map_err(|_| return FastenerErr::FailedFetchingPackage);
                return Ok(());
            }
        }
    }
}

fn main() {
    let fastener: Fastener = argh::from_env();
    match fastener.run() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("fastener ERR: {:?}", err);
        }
    }
}
