use crate::RootOpt;
use anyhow::Error;
use aoc_client::AocClient;
use clap::Parser;
use itertools::Itertools;
use std::path::PathBuf;

/// Wrapper around the Advent of Code API Client
pub struct Client {
    pub client: AocClient,
    pub assignment_path: PathBuf,
    pub input_path: PathBuf,
    pub year: u16,
    pub day: u8,
    pub part: u8,
}

#[derive(Parser, Debug, Clone)]
pub struct DownloadCommand {
    /// Force download even if files already exist
    #[arg(short, long)]
    pub force: bool,
}

#[derive(Parser, Debug, Clone)]
pub struct SubmitCommand {
    pub answer: i64,
}

impl Client {
    pub fn new(opt: &RootOpt) -> Result<Self, Error> {
        let assignment_path = PathBuf::from(format!("./assignments/day{:02}.md", opt.day));
        let input_path = PathBuf::from(format!("./input/day{:02}.txt", opt.day));

        let client = AocClient::builder()
            .session_cookie_from_default_locations()?
            .year(opt.year as i32)?
            .day(opt.day as u32)?
            .puzzle_filename(&assignment_path)
            .build()?;

        Ok(Self {
            client,
            assignment_path,
            input_path,
            year: opt.year,
            day: opt.day,
            part: opt.part,
        })
    }

    /// Download the assignment and input data
    pub fn download(&self) -> Result<(), Error> {
        if !self.assignment_path.exists() {
            self.client.save_puzzle_markdown()?;
        }

        if !self.input_path.exists() {
            let input = self.client.get_input()?;
            std::fs::write(format!("./input/day{:02}.txt", self.day), input)?;
        }
        Ok(())
    }
    pub fn ensure_ready(&self, day: u8) -> Result<(), Error> {
        if !solution_path(day).exists() {
            let downloader = DownloadCommand { force: false };
            let dl_opt = RootOpt {
                year: self.year,
                day: self.day,
                part: self.part,
                data: false,
                command: None,
            };
            log::warn!("Since a new code-file was created, you may need to restart your language-server / restart your watch command!");
            return downloader.run(&dl_opt);
        } else {
            // ensure both assignment & input are present even if the code-file was present
            return self.download();
        }
    }

    /// Delete downloaded files
    pub fn clear(&self) -> Result<(), Error> {
        if self.assignment_path.exists() {
            std::fs::remove_file(&self.assignment_path)?;
        }

        if self.input_path.exists() {
            std::fs::remove_file(&self.input_path)?;
        }
        Ok(())
    }

    /// Get the input for the day. If the input file doesn't exist, download it.
    pub fn get_input(&self, day: u8) -> Result<String, Error> {
        self.ensure_ready(day)?;

        let input = std::fs::read_to_string(&self.input_path)?;
        Ok(input)
    }
}

fn solution_path(day: u8) -> PathBuf {
    PathBuf::from(format!("src/puzzle/day_{:02}.rs", day))
}

impl DownloadCommand {
    pub fn run(&self, opt: &RootOpt) -> Result<(), Error> {
        log::info!("Running download command");

        let client = Client::new(opt)?;
        if self.force {
            client.clear()?;
        }
        client.download()?;
        Self::create_solution_file(opt.day)?;

        Ok(())
    }

    fn create_solution_file(day: u8) -> Result<(), Error> {
        let template_path = PathBuf::from("src/puzzle/day_00.rs");
        let path = solution_path(day);

        if std::fs::exists(&path)? {
            log::info!("Puzzle file exists at {:?}", path);
            return Ok(());
        }

        // hackity hack - relies on very specific file layout in template
        log::info!("Creating puzzle file at {:?}", path);
        let day_text = format!("Day{:02}", day);
        let day_lower = day_text.to_lowercase();
        let data: String = String::from_utf8(std::fs::read(template_path)?)?
            // replace type name with specific day
            .replace("Day00", &day_text)
            .replace("day00", &day_lower)
            .lines()
            // remove the two comment lines at the top of the file
            .skip(2)
            .collect_vec()
            .join("\n");
        std::fs::write(path, data)?;

        Ok(())
    }
}

impl SubmitCommand {
    pub fn run(&self, opt: &RootOpt) -> Result<(), Error> {
        log::info!("Running submit command");
        let client = Client::new(opt)?;
        let res = client.client.submit_answer(opt.part as i64, self.answer)?;
        println!("{:?}", res);
        Ok(())
    }
}
