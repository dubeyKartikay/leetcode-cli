//! Leetcode data models
use colored::Colorize;
use serde::{Serialize, Deserialize};
use super::schemas::problems;
use crate::helper::HTML;

/// Problem model
#[derive(AsChangeset, Clone, Identifiable, Insertable, Queryable, Serialize, Debug)]
#[table_name = "problems"]
pub struct Problem {
    pub category: String,
    pub fid: i32,
    pub id: i32,
    pub level: i32,
    pub locked: bool,
    pub name: String,
    pub percent: f32,
    pub slug: String,
    pub starred: bool,
    pub status: String,
    pub desc: String,
}

static DONE: &'static str = " ✔";
static ETC: &'static str = "...";
static LOCK: &'static str = "🔒";
static NDONE: &'static str = "✘";
static SPACE: &'static str = " ";
impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let space_2 = SPACE.repeat(2);
        let mut lock = space_2.as_str();
        let mut done = space_2.normal();
        let mut id = "".to_string();
        let mut name = "".to_string();
        let mut level = "".normal();

        if self.locked { lock = LOCK };
        if self.status == "ac".to_string() {
            done = DONE.green().bold();
        } else if self.status == "notac" {
            done = NDONE.green().bold();
        }

        match self.fid.to_string().len() {
            1 => {
                id.push_str(&SPACE.repeat(2));
                id.push_str(&self.fid.to_string());
                id.push_str(&SPACE.repeat(1));
            },
            2 => {
                id.push_str(&SPACE.repeat(1));
                id.push_str(&self.fid.to_string());
                id.push_str(&SPACE.repeat(1));
            },
            3 => {
                id.push_str(&SPACE.repeat(1));
                id.push_str(&self.fid.to_string());
            },
            4 => {
                id.push_str(&self.fid.to_string());
            },
            _ => {
                id.push_str(&space_2);
                id.push_str(&space_2);
            }
        }

        if &self.name.len() < &60_usize {
            name.push_str(&self.name);
            name.push_str(&SPACE.repeat(60 - &self.name.len()));
        } else {
            name.push_str(&self.name[..49]);
            name = name.trim_end().to_string();
            name.push_str(ETC);
            name.push_str(&SPACE.repeat(60 - name.len()));
        }

        level = match self.level {
            1 => "Easy  ".bright_green(),
            2 => "Medium".bright_yellow(),
            3 => "Hard  ".bright_red(),
            _ => level
        };
        
        write!(
            f,
            "  {} {} [{}] {} {} ({} %)",
            lock, done, id, name, level,
            &self.percent.to_string()[0..5]
        )
    }
}

/// desc model
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Question {
    pub content: String,
    pub stats: Stats,
    pub defs: CodeDefintion,
    pub case: String,
    pub metadata: MetaData,
    pub test: bool,
    pub t_content: String,
}

impl std::fmt::Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.content.render())
    }
}


use question::*;
/// deps of Question
mod question {
    use serde::{Serialize, Deserialize};

    /// Code samples
    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct CodeDefintion(pub Vec<CodeDefintionInner>);

    /// CodeDefinition Inner struct
    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct CodeDefintionInner {
        pub value: String,
        pub text: String,
        #[serde(alias = "defaultCode")]
        pub code: String,
    }

    /// Question status
    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct Stats {
        #[serde(alias = "totalAccepted")]
        tac: String,
        #[serde(alias = "totalSubmission")]
        tsm: String,
        #[serde(alias = "totalAcceptedRaw")]
        tacr: i32,
        #[serde(alias = "totalSubmissionRaw")]
        tsmr: i32,
        #[serde(alias = "acRate")]
        rate: String
    }
    
    /// Algorithm metadata
    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct MetaData {
        pub name: String,
        pub params: Vec<Param>,
        pub r#return: Return,
    }

    /// MetaData nested fields
    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct Param {
        pub name: String,
        pub r#type: String
    }

    /// MetaData nested fields
    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct Return {
        pub r#type: String
    }
}


/// run_code Result
#[derive(Debug, Deserialize)]
pub struct RunCode {
    #[serde(default)]
    pub interpret_id: String,
    #[serde(default)]
    pub test_case: String,
    #[serde(default)]
    pub submission_id: i64,
}

use super::parser::ssr;
/// verify result model
#[derive(Debug, Deserialize)]
pub struct VerifyResult {
    pub state: String,
    #[serde(skip)]
    pub name: String,
    #[serde(skip)]
    pub data_input: String,
    #[serde(default)]
    lang: String,
    #[serde(default)]
    pretty_lang: String,
    #[serde(default)]
    submission_id: String,
    #[serde(default)]
    run_success: bool,
    #[serde(default)]
    correct_answer: bool,
    #[serde(default, deserialize_with = "ssr")]
    code_answer: Vec<String>,
    #[serde(default, deserialize_with = "ssr")]
    code_output: Vec<String>,
    #[serde(default)]
    std_output: String,
    
    // flatten
    #[serde(flatten, default)]
    info: VerifyInfo,
    #[serde(flatten, default)]
    status: VerifyStatus,
    #[serde(flatten, default)]
    analyse: Analyse,
    #[serde(flatten, default)]
    expected: Expected,
    #[serde(flatten, default)]
    error: CompileError,
    #[serde(flatten, default)]
    submit: Submit,
}

impl std::fmt::Display for VerifyResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ca = match &self.code_answer.len() {
            1 => self.code_answer[0].to_string(),
            _ => self.code_answer.join(""),
        };

        let eca = match &self.expected.expected_code_answer.len() {
            1 => self.expected.expected_code_answer[0].to_string(),
            _ => self.expected.expected_code_answer.join(""),
        };

        debug!("{:#?}", &self);

        match &self.status.status_code {
            10 => match self.correct_answer {
                // Pass Test
                true => write!(
                    f,
                    "\n  {}{}{}\n{}{}{}{}{}{}\n",
                    &self.status.status_msg.green().bold(),
                    "       Runtime: ".dimmed(),
                    &self.status.status_runtime.dimmed(),
                    "\n  Your input:    ",
                    &self.data_input.replace("\n", ", "),
                    "\n  Output:        ",
                    ca,
                    "\n  Expected:      ",
                    eca,
                ),
                false => match &self.submit.compare_result.len() > &0 {
                    // Submit Successfully
                    true => write!(
                        f,
                        "\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}.\n",
                        "  Success\n\n".green().bold(),
                        "  Runtime: ".dimmed(),
                        &self.status.status_runtime.bold(),
                        ", faster than ",
                        &self.analyse.runtime_percentile.unwrap().to_string().bold(),
                        "% ".bold(),
                        "of ",
                        &self.pretty_lang,
                        "online submissions for ",
                        &self.name,
                        ".\n\n",
                        "  Memory Usage: ".dimmed(),
                        &self.status.status_memory.bold(),
                        ", less than ",
                        &self.analyse.memory_percentile.unwrap().to_string().bold(),
                        "% ".bold(),
                        "of ",
                        &self.pretty_lang,
                        "online submissions for ",
                        &self.name,
                    ),
                    // Wrong Answer
                    false => write!(
                        f,
                        "\n{}{}{}\n{}{}{}{}{}{}\n",
                        "  Wrong Answer".red().bold(),
                        "   Runtime: ".dimmed(),
                        &self.status.status_runtime.dimmed(),
                        "\n  Your input:    ",
                        &self.data_input.replace("\n", ", "),
                        "\n  Output:        ",
                        ca,
                        "\n  Expected:      ",
                        eca,
                    ),
                }
            },
            // Output Timeout Exceeded
            13 => write!(
                f,
                "\n{}:\n\n{:?}\n",
                &self.status.status_msg.yellow().bold(),
                &self.code_output,
            ),
            // Compile Error
            20 => write!(
                f,
                "\n{}:\n\n{}\n",
                &self.status.status_msg.red().bold(),
                &self.error.full_compile_error
            ),
            _ => write!(f, "{}", "\nUnKnow Error...\n".red().bold())
        }
    }
}

use verify::*;
mod verify {
    use serde::Deserialize;
    use super::super::parser::ssr;
    
    #[derive(Debug, Default, Deserialize)]
    pub struct Submit {
        #[serde(default)]
        question_id: String,
        #[serde(default)]
        last_testcase: String,
        #[serde(default)]
        pub compare_result: String,
    }
    
    #[derive(Debug, Default, Deserialize)]
    pub struct VerifyInfo {
        #[serde(default)]
        memory: i64,
        #[serde(default)]
        elapsed_time: i64,
        #[serde(default)]
        task_finish_time: i64,
    }
    
    #[derive(Debug, Default, Deserialize)]
    pub struct Analyse {
        #[serde(default)]
        pub total_correct: Option<i32>,
        #[serde(default)]
        pub total_testcases: Option<i32>,
        #[serde(default)]
        pub runtime_percentile: Option<i32>,
        #[serde(default)]
        pub memory_percentile: Option<i32>,
    }
    
    #[derive(Debug, Default, Deserialize)]
    pub struct VerifyStatus {
        #[serde(default)]
        pub status_code: i32,
        #[serde(default)]
        pub status_msg: String,
        #[serde(default)]
        pub status_memory: String,
        #[serde(default)]
        pub status_runtime: String,
    }
    
    #[derive(Debug, Default, Deserialize)]
    pub struct CompileError {
        #[serde(default)]
        compile_error: String,
        #[serde(default)]
        pub full_compile_error: String,
    }
    
    #[derive(Debug, Default, Deserialize)]
    pub struct Expected {
        #[serde(default)]
        expected_status_code: i32,
        #[serde(default)]
        expected_lang: String,
        #[serde(default)]
        expected_run_success: bool,
        #[serde(default)]
        expected_status_runtime: String,
        #[serde(default)]
        expected_memory: i64,
        #[serde(default, deserialize_with = "ssr")]
        expected_code_output: Vec<String>,
        #[serde(default)]
        expected_elapsed_time: i64,
        #[serde(default)]
        expected_task_finish_time: i64,
        #[serde(default, deserialize_with = "ssr")]
        pub expected_code_answer: Vec<String>
    }
}