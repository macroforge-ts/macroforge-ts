//! TUI application state

use std::collections::VecDeque;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

/// Task status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
}

/// A task in the task list
#[derive(Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub name: String,
    pub command: String,
    pub status: TaskStatus,
}

/// Log level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Success,
    Warning,
    Error,
}

/// Log entry
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub level: LogLevel,
    pub message: String,
}

/// Currently focused panel
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusedPanel {
    TaskList,
    OutputLog,
}

/// Messages from task runner to UI
#[derive(Debug)]
pub enum TaskMessage {
    Started(usize),
    Output(String),
    Completed(usize),
    Failed(usize, String),
}

/// Main application state
pub struct App {
    pub tasks: Vec<Task>,
    pub selected_task: usize,
    pub logs: VecDeque<LogEntry>,
    pub max_logs: usize,
    pub focused: FocusedPanel,
    pub running: bool,
    pub should_quit: bool,
    pub task_sender: Sender<TaskMessage>,
    pub task_receiver: Receiver<TaskMessage>,
    pub current_task: Option<usize>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// Create a new app
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            tasks: Vec::new(),
            selected_task: 0,
            logs: VecDeque::with_capacity(1000),
            max_logs: 1000,
            focused: FocusedPanel::TaskList,
            running: false,
            should_quit: false,
            task_sender: sender,
            task_receiver: receiver,
            current_task: None,
        }
    }

    /// Add a task
    pub fn add_task(&mut self, name: String, command: String) {
        self.tasks.push(Task {
            id: self.tasks.len(),
            name,
            command,
            status: TaskStatus::Pending,
        });
    }

    /// Log a message
    pub fn log(&mut self, level: LogLevel, message: impl Into<String>) {
        self.logs.push_back(LogEntry {
            level,
            message: message.into(),
        });
        if self.logs.len() > self.max_logs {
            self.logs.pop_front();
        }
    }

    /// Move selection up
    pub fn select_previous(&mut self) {
        if self.selected_task > 0 {
            self.selected_task -= 1;
        }
    }

    /// Move selection down
    pub fn select_next(&mut self) {
        if self.selected_task < self.tasks.len().saturating_sub(1) {
            self.selected_task += 1;
        }
    }

    /// Toggle focused panel
    pub fn toggle_focus(&mut self) {
        self.focused = match self.focused {
            FocusedPanel::TaskList => FocusedPanel::OutputLog,
            FocusedPanel::OutputLog => FocusedPanel::TaskList,
        };
    }

    /// Initialize with default tasks
    pub fn init_default_tasks(&mut self) {
        self.add_task(
            "Prepare release".into(),
            "prep --repos all --dry-run".into(),
        );
        self.add_task(
            "Commit release".into(),
            "commit --repos all --dry-run".into(),
        );
        self.add_task("Run diagnostics".into(), "diagnostics".into());
        self.add_task("Generate docs".into(), "docs all".into());
        self.add_task("Build all".into(), "build --repos all".into());
    }

    /// Run the selected task
    pub fn run_selected_task(&mut self) {
        if self.running {
            self.log(LogLevel::Warning, "A task is already running");
            return;
        }

        if self.selected_task >= self.tasks.len() {
            return;
        }

        let task = &self.tasks[self.selected_task];
        let task_id = task.id;
        let command = task.command.clone();

        self.running = true;
        self.current_task = Some(task_id);
        self.tasks[task_id].status = TaskStatus::Running;

        let sender = self.task_sender.clone();

        // Spawn thread to run the command
        thread::spawn(move || {
            let _ = sender.send(TaskMessage::Started(task_id));

            // Build the command - run mf with the task's command args
            let exe = std::env::current_exe().unwrap_or_else(|_| "mf".into());
            let args: Vec<&str> = command.split_whitespace().collect();

            let result = Command::new(&exe)
                .args(&args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn();

            match result {
                Ok(mut child) => {
                    // Read stdout
                    if let Some(stdout) = child.stdout.take() {
                        let reader = BufReader::new(stdout);
                        for line in reader.lines().map_while(Result::ok) {
                            let _ = sender.send(TaskMessage::Output(line));
                        }
                    }

                    // Read stderr
                    if let Some(stderr) = child.stderr.take() {
                        let reader = BufReader::new(stderr);
                        for line in reader.lines().map_while(Result::ok) {
                            let _ = sender.send(TaskMessage::Output(format!("! {}", line)));
                        }
                    }

                    // Wait for completion
                    match child.wait() {
                        Ok(status) if status.success() => {
                            let _ = sender.send(TaskMessage::Completed(task_id));
                        }
                        Ok(status) => {
                            let _ = sender.send(TaskMessage::Failed(
                                task_id,
                                format!("Exit code: {}", status.code().unwrap_or(-1)),
                            ));
                        }
                        Err(e) => {
                            let _ = sender.send(TaskMessage::Failed(task_id, e.to_string()));
                        }
                    }
                }
                Err(e) => {
                    let _ = sender.send(TaskMessage::Failed(task_id, e.to_string()));
                }
            }
        });
    }

    /// Process pending messages from task runner
    pub fn process_messages(&mut self) {
        while let Ok(msg) = self.task_receiver.try_recv() {
            match msg {
                TaskMessage::Started(id) => {
                    self.log(LogLevel::Info, format!("Started: {}", self.tasks[id].name));
                }
                TaskMessage::Output(line) => {
                    // Strip ANSI codes for cleaner display
                    let clean_line = strip_ansi(&line);
                    self.log(LogLevel::Info, clean_line);
                }
                TaskMessage::Completed(id) => {
                    self.tasks[id].status = TaskStatus::Completed;
                    self.running = false;
                    self.current_task = None;
                    self.log(
                        LogLevel::Success,
                        format!("Completed: {}", self.tasks[id].name),
                    );
                }
                TaskMessage::Failed(id, error) => {
                    self.tasks[id].status = TaskStatus::Failed(error.clone());
                    self.running = false;
                    self.current_task = None;
                    self.log(
                        LogLevel::Error,
                        format!("Failed: {} - {}", self.tasks[id].name, error),
                    );
                }
            }
        }
    }
}

/// Strip ANSI escape codes from a string
fn strip_ansi(s: &str) -> String {
    let re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    re.replace_all(s, "").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_new_initial_state() {
        let app = App::new();

        assert_eq!(app.tasks.len(), 0, "should start with no tasks");
        assert_eq!(app.selected_task, 0, "should start with selection at 0");
        assert_eq!(app.logs.len(), 0, "should start with no logs");
        assert_eq!(app.max_logs, 1000, "should have max_logs set to 1000");
        assert_eq!(
            app.focused,
            FocusedPanel::TaskList,
            "should start focused on task list"
        );
        assert!(!app.running, "should not be running initially");
        assert!(!app.should_quit, "should_quit should be false");
        assert_eq!(app.current_task, None, "should have no current task");
        assert_eq!(
            app.logs.capacity(),
            1000,
            "logs should have capacity of 1000"
        );
    }

    #[test]
    fn test_app_default_matches_new() {
        let app1 = App::new();
        let app2 = App::default();

        assert_eq!(app1.tasks.len(), app2.tasks.len());
        assert_eq!(app1.selected_task, app2.selected_task);
        assert_eq!(app1.focused, app2.focused);
        assert_eq!(app1.running, app2.running);
        assert_eq!(app1.should_quit, app2.should_quit);
    }

    #[test]
    fn test_add_task_single() {
        let mut app = App::new();

        app.add_task("Test Task".into(), "test command".into());

        assert_eq!(app.tasks.len(), 1);
        assert_eq!(app.tasks[0].id, 0);
        assert_eq!(app.tasks[0].name, "Test Task");
        assert_eq!(app.tasks[0].command, "test command");
        assert_eq!(app.tasks[0].status, TaskStatus::Pending);
    }

    #[test]
    fn test_add_task_multiple() {
        let mut app = App::new();

        app.add_task("Task 1".into(), "cmd1".into());
        app.add_task("Task 2".into(), "cmd2".into());
        app.add_task("Task 3".into(), "cmd3".into());

        assert_eq!(app.tasks.len(), 3);

        // Verify each task has correct ID
        assert_eq!(app.tasks[0].id, 0);
        assert_eq!(app.tasks[1].id, 1);
        assert_eq!(app.tasks[2].id, 2);

        // Verify names
        assert_eq!(app.tasks[0].name, "Task 1");
        assert_eq!(app.tasks[1].name, "Task 2");
        assert_eq!(app.tasks[2].name, "Task 3");

        // Verify all are pending
        for task in &app.tasks {
            assert_eq!(task.status, TaskStatus::Pending);
        }
    }

    #[test]
    fn test_add_task_empty_strings() {
        let mut app = App::new();

        app.add_task("".into(), "".into());

        assert_eq!(app.tasks.len(), 1);
        assert_eq!(app.tasks[0].name, "");
        assert_eq!(app.tasks[0].command, "");
    }

    #[test]
    fn test_log_info() {
        let mut app = App::new();

        app.log(LogLevel::Info, "Info message");

        assert_eq!(app.logs.len(), 1);
        assert_eq!(app.logs[0].level, LogLevel::Info);
        assert_eq!(app.logs[0].message, "Info message");
    }

    #[test]
    fn test_log_all_levels() {
        let mut app = App::new();

        app.log(LogLevel::Info, "Info");
        app.log(LogLevel::Success, "Success");
        app.log(LogLevel::Warning, "Warning");
        app.log(LogLevel::Error, "Error");

        assert_eq!(app.logs.len(), 4);
        assert_eq!(app.logs[0].level, LogLevel::Info);
        assert_eq!(app.logs[1].level, LogLevel::Success);
        assert_eq!(app.logs[2].level, LogLevel::Warning);
        assert_eq!(app.logs[3].level, LogLevel::Error);
    }

    #[test]
    fn test_log_string_conversion() {
        let mut app = App::new();

        // Test with &str
        app.log(LogLevel::Info, "static str");

        // Test with String
        app.log(LogLevel::Info, String::from("owned string"));

        // Test with format!
        let value = 42;
        app.log(LogLevel::Info, format!("value: {}", value));

        assert_eq!(app.logs.len(), 3);
        assert_eq!(app.logs[0].message, "static str");
        assert_eq!(app.logs[1].message, "owned string");
        assert_eq!(app.logs[2].message, "value: 42");
    }

    #[test]
    fn test_log_max_capacity() {
        let mut app = App::new();
        app.max_logs = 5; // Set lower limit for testing

        // Add more logs than max_logs
        for i in 0..10 {
            app.log(LogLevel::Info, format!("Log {}", i));
        }

        assert_eq!(app.logs.len(), 5, "should not exceed max_logs");

        // Verify oldest logs were removed (should have logs 5-9)
        assert_eq!(app.logs[0].message, "Log 5");
        assert_eq!(app.logs[4].message, "Log 9");
    }

    #[test]
    fn test_log_exactly_at_max_capacity() {
        let mut app = App::new();
        app.max_logs = 3;

        app.log(LogLevel::Info, "Log 0");
        app.log(LogLevel::Info, "Log 1");
        app.log(LogLevel::Info, "Log 2");

        assert_eq!(app.logs.len(), 3);

        app.log(LogLevel::Info, "Log 3");

        assert_eq!(app.logs.len(), 3);
        assert_eq!(app.logs[0].message, "Log 1");
        assert_eq!(app.logs[2].message, "Log 3");
    }

    #[test]
    fn test_select_previous_basic() {
        let mut app = App::new();
        app.add_task("Task 0".into(), "cmd0".into());
        app.add_task("Task 1".into(), "cmd1".into());
        app.add_task("Task 2".into(), "cmd2".into());

        app.selected_task = 2;
        app.select_previous();
        assert_eq!(app.selected_task, 1);

        app.select_previous();
        assert_eq!(app.selected_task, 0);
    }

    #[test]
    fn test_select_previous_at_boundary() {
        let mut app = App::new();
        app.add_task("Task 0".into(), "cmd0".into());
        app.add_task("Task 1".into(), "cmd1".into());

        app.selected_task = 0;
        app.select_previous();

        assert_eq!(app.selected_task, 0, "should stay at 0 when at boundary");
    }

    #[test]
    fn test_select_previous_multiple_times_at_boundary() {
        let mut app = App::new();
        app.add_task("Task".into(), "cmd".into());

        app.selected_task = 0;
        app.select_previous();
        app.select_previous();
        app.select_previous();

        assert_eq!(app.selected_task, 0, "should remain at 0");
    }

    #[test]
    fn test_select_previous_empty_tasks() {
        let mut app = App::new();

        app.selected_task = 0;
        app.select_previous();

        assert_eq!(app.selected_task, 0);
    }

    #[test]
    fn test_select_next_basic() {
        let mut app = App::new();
        app.add_task("Task 0".into(), "cmd0".into());
        app.add_task("Task 1".into(), "cmd1".into());
        app.add_task("Task 2".into(), "cmd2".into());

        app.selected_task = 0;
        app.select_next();
        assert_eq!(app.selected_task, 1);

        app.select_next();
        assert_eq!(app.selected_task, 2);
    }

    #[test]
    fn test_select_next_at_boundary() {
        let mut app = App::new();
        app.add_task("Task 0".into(), "cmd0".into());
        app.add_task("Task 1".into(), "cmd1".into());

        app.selected_task = 1; // Last task
        app.select_next();

        assert_eq!(
            app.selected_task, 1,
            "should stay at last task when at boundary"
        );
    }

    #[test]
    fn test_select_next_multiple_times_at_boundary() {
        let mut app = App::new();
        app.add_task("Task 0".into(), "cmd0".into());
        app.add_task("Task 1".into(), "cmd1".into());

        app.selected_task = 1;
        app.select_next();
        app.select_next();
        app.select_next();

        assert_eq!(app.selected_task, 1, "should remain at last task");
    }

    #[test]
    fn test_select_next_empty_tasks() {
        let mut app = App::new();

        app.selected_task = 0;
        app.select_next();

        assert_eq!(app.selected_task, 0, "should stay at 0 when no tasks");
    }

    #[test]
    fn test_select_next_single_task() {
        let mut app = App::new();
        app.add_task("Only Task".into(), "cmd".into());

        app.selected_task = 0;
        app.select_next();

        assert_eq!(app.selected_task, 0, "should stay at 0 with single task");
    }

    #[test]
    fn test_navigation_sequence() {
        let mut app = App::new();
        for i in 0..5 {
            app.add_task(format!("Task {}", i), format!("cmd{}", i));
        }

        // Start at 0
        assert_eq!(app.selected_task, 0);

        // Go down to 4
        app.select_next();
        app.select_next();
        app.select_next();
        app.select_next();
        assert_eq!(app.selected_task, 4);

        // Try to go beyond
        app.select_next();
        assert_eq!(app.selected_task, 4);

        // Go back up to 0
        app.select_previous();
        app.select_previous();
        app.select_previous();
        app.select_previous();
        assert_eq!(app.selected_task, 0);

        // Try to go before
        app.select_previous();
        assert_eq!(app.selected_task, 0);
    }

    #[test]
    fn test_toggle_focus_from_task_list() {
        let mut app = App::new();

        assert_eq!(app.focused, FocusedPanel::TaskList);

        app.toggle_focus();

        assert_eq!(app.focused, FocusedPanel::OutputLog);
    }

    #[test]
    fn test_toggle_focus_from_output_log() {
        let mut app = App::new();
        app.focused = FocusedPanel::OutputLog;

        app.toggle_focus();

        assert_eq!(app.focused, FocusedPanel::TaskList);
    }

    #[test]
    fn test_toggle_focus_multiple_times() {
        let mut app = App::new();

        assert_eq!(app.focused, FocusedPanel::TaskList);

        app.toggle_focus();
        assert_eq!(app.focused, FocusedPanel::OutputLog);

        app.toggle_focus();
        assert_eq!(app.focused, FocusedPanel::TaskList);

        app.toggle_focus();
        assert_eq!(app.focused, FocusedPanel::OutputLog);

        app.toggle_focus();
        assert_eq!(app.focused, FocusedPanel::TaskList);
    }

    #[test]
    fn test_init_default_tasks() {
        let mut app = App::new();

        app.init_default_tasks();

        assert_eq!(app.tasks.len(), 5, "should add 5 default tasks");

        // Verify task IDs
        for (i, task) in app.tasks.iter().enumerate() {
            assert_eq!(task.id, i);
            assert_eq!(task.status, TaskStatus::Pending);
        }

        // Verify specific tasks
        assert_eq!(app.tasks[0].name, "Prepare release");
        assert_eq!(app.tasks[0].command, "prep --repos all --dry-run");

        assert_eq!(app.tasks[1].name, "Commit release");
        assert_eq!(app.tasks[1].command, "commit --repos all --dry-run");

        assert_eq!(app.tasks[2].name, "Run diagnostics");
        assert_eq!(app.tasks[2].command, "diagnostics");

        assert_eq!(app.tasks[3].name, "Generate docs");
        assert_eq!(app.tasks[3].command, "docs all");

        assert_eq!(app.tasks[4].name, "Build all");
        assert_eq!(app.tasks[4].command, "build --repos all");
    }

    #[test]
    fn test_init_default_tasks_multiple_calls() {
        let mut app = App::new();

        app.init_default_tasks();
        app.init_default_tasks();

        assert_eq!(app.tasks.len(), 10, "should add tasks each time called");

        // Verify IDs are sequential
        for (i, task) in app.tasks.iter().enumerate() {
            assert_eq!(task.id, i);
        }
    }

    #[test]
    fn test_init_default_tasks_after_adding_custom_tasks() {
        let mut app = App::new();

        app.add_task("Custom Task".into(), "custom cmd".into());
        app.init_default_tasks();

        assert_eq!(app.tasks.len(), 6);
        assert_eq!(app.tasks[0].name, "Custom Task");
        assert_eq!(app.tasks[1].name, "Prepare release");
    }

    #[test]
    fn test_task_status_pending() {
        let status = TaskStatus::Pending;
        assert_eq!(status, TaskStatus::Pending);
    }

    #[test]
    fn test_task_status_running() {
        let status = TaskStatus::Running;
        assert_eq!(status, TaskStatus::Running);
    }

    #[test]
    fn test_task_status_completed() {
        let status = TaskStatus::Completed;
        assert_eq!(status, TaskStatus::Completed);
    }

    #[test]
    fn test_task_status_failed() {
        let status = TaskStatus::Failed("error message".into());
        match status {
            TaskStatus::Failed(msg) => assert_eq!(msg, "error message"),
            _ => panic!("Expected Failed variant"),
        }
    }

    #[test]
    fn test_task_status_failed_empty_message() {
        let status = TaskStatus::Failed(String::new());
        match status {
            TaskStatus::Failed(msg) => assert_eq!(msg, ""),
            _ => panic!("Expected Failed variant"),
        }
    }

    #[test]
    fn test_task_status_equality() {
        assert_eq!(TaskStatus::Pending, TaskStatus::Pending);
        assert_eq!(TaskStatus::Running, TaskStatus::Running);
        assert_eq!(TaskStatus::Completed, TaskStatus::Completed);
        assert_eq!(
            TaskStatus::Failed("error".into()),
            TaskStatus::Failed("error".into())
        );

        assert_ne!(TaskStatus::Pending, TaskStatus::Running);
        assert_ne!(TaskStatus::Running, TaskStatus::Completed);
        assert_ne!(
            TaskStatus::Failed("error1".into()),
            TaskStatus::Failed("error2".into())
        );
    }

    #[test]
    fn test_task_status_clone() {
        let status1 = TaskStatus::Failed("test error".into());
        let status2 = status1.clone();

        assert_eq!(status1, status2);
    }

    #[test]
    fn test_log_level_info() {
        let level = LogLevel::Info;
        assert_eq!(level, LogLevel::Info);
    }

    #[test]
    fn test_log_level_success() {
        let level = LogLevel::Success;
        assert_eq!(level, LogLevel::Success);
    }

    #[test]
    fn test_log_level_warning() {
        let level = LogLevel::Warning;
        assert_eq!(level, LogLevel::Warning);
    }

    #[test]
    fn test_log_level_error() {
        let level = LogLevel::Error;
        assert_eq!(level, LogLevel::Error);
    }

    #[test]
    fn test_log_level_equality() {
        assert_eq!(LogLevel::Info, LogLevel::Info);
        assert_eq!(LogLevel::Success, LogLevel::Success);
        assert_eq!(LogLevel::Warning, LogLevel::Warning);
        assert_eq!(LogLevel::Error, LogLevel::Error);

        assert_ne!(LogLevel::Info, LogLevel::Success);
        assert_ne!(LogLevel::Warning, LogLevel::Error);
    }

    #[test]
    fn test_log_level_copy() {
        let level1 = LogLevel::Info;
        let level2 = level1;

        // Both should be usable (Copy trait)
        assert_eq!(level1, LogLevel::Info);
        assert_eq!(level2, LogLevel::Info);
    }

    #[test]
    fn test_focused_panel_equality() {
        assert_eq!(FocusedPanel::TaskList, FocusedPanel::TaskList);
        assert_eq!(FocusedPanel::OutputLog, FocusedPanel::OutputLog);
        assert_ne!(FocusedPanel::TaskList, FocusedPanel::OutputLog);
    }

    #[test]
    fn test_focused_panel_copy() {
        let panel1 = FocusedPanel::TaskList;
        let panel2 = panel1;

        // Both should be usable (Copy trait)
        assert_eq!(panel1, FocusedPanel::TaskList);
        assert_eq!(panel2, FocusedPanel::TaskList);
    }

    #[test]
    fn test_task_clone() {
        let task1 = Task {
            id: 1,
            name: "Test".into(),
            command: "test cmd".into(),
            status: TaskStatus::Running,
        };

        let task2 = task1.clone();

        assert_eq!(task1.id, task2.id);
        assert_eq!(task1.name, task2.name);
        assert_eq!(task1.command, task2.command);
        assert_eq!(task1.status, task2.status);
    }

    #[test]
    fn test_log_entry_clone() {
        let entry1 = LogEntry {
            level: LogLevel::Error,
            message: "Test error".into(),
        };

        let entry2 = entry1.clone();

        assert_eq!(entry1.level, entry2.level);
        assert_eq!(entry1.message, entry2.message);
    }

    #[test]
    fn test_strip_ansi_no_codes() {
        let result = strip_ansi("plain text");
        assert_eq!(result, "plain text");
    }

    #[test]
    fn test_strip_ansi_with_codes() {
        let result = strip_ansi("\x1b[31mred text\x1b[0m");
        assert_eq!(result, "red text");
    }

    #[test]
    fn test_strip_ansi_multiple_codes() {
        let result = strip_ansi("\x1b[1m\x1b[31mbold red\x1b[0m\x1b[32mgreen\x1b[0m");
        assert_eq!(result, "bold redgreen");
    }

    #[test]
    fn test_strip_ansi_empty_string() {
        let result = strip_ansi("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_run_selected_task_when_running() {
        let mut app = App::new();
        app.add_task("Task".into(), "cmd".into());
        app.running = true;

        let initial_logs = app.logs.len();
        app.run_selected_task();

        // Should log warning and not start task
        assert_eq!(app.logs.len(), initial_logs + 1);
        assert_eq!(app.logs[0].level, LogLevel::Warning);
        assert_eq!(app.logs[0].message, "A task is already running");
    }

    #[test]
    fn test_run_selected_task_out_of_bounds() {
        let mut app = App::new();
        app.add_task("Task".into(), "cmd".into());
        app.selected_task = 10; // Out of bounds

        app.run_selected_task();

        // Should return early without errors
        assert!(!app.running);
        assert_eq!(app.current_task, None);
    }
}
