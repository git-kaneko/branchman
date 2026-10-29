use crate::git::{self, Branch};
use anyhow::Result;
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppMode {
    Normal,
    ConfirmDelete,
}

pub struct App {
    pub branches: Vec<Branch>,
    pub selected_index: usize,
    pub selected_branches: HashSet<String>,
    pub mode: AppMode,
    pub repo_path: PathBuf,
}

impl App {
    pub fn new(repo_path: PathBuf) -> Result<Self> {
        let branches = git::get_branches(&repo_path)?;

        Ok(Self {
            branches,
            selected_index: 0,
            selected_branches: HashSet::new(),
            mode: AppMode::Normal,
            repo_path,
        })
    }

    pub fn reload_branches(&mut self) -> Result<()> {
        self.branches = git::get_branches(&self.repo_path)?;
        self.selected_branches.clear();

        if self.selected_index >= self.branches.len() && !self.branches.is_empty() {
            self.selected_index = self.branches.len() - 1;
        }

        Ok(())
    }

    pub fn next(&mut self) {
        if self.branches.is_empty() {
            return;
        }

        if self.selected_index < self.branches.len() - 1 {
            self.selected_index += 1;
        } else {
            self.selected_index = 0;
        }
    }

    pub fn previous(&mut self) {
        if self.branches.is_empty() {
            return;
        }

        if self.selected_index > 0 {
            self.selected_index -= 1;
        } else {
            self.selected_index = self.branches.len() - 1;
        }
    }

    pub fn jump_to_top(&mut self) {
        self.selected_index = 0;
    }

    pub fn jump_to_bottom(&mut self) {
        if !self.branches.is_empty() {
            self.selected_index = self.branches.len() - 1;
        }
    }

    pub fn toggle_selection(&mut self) {
        if self.branches.is_empty() {
            return;
        }

        let branch = &self.branches[self.selected_index];

        if !self.is_protected_branch(&branch.name) {
            if self.selected_branches.contains(&branch.name) {
                self.selected_branches.remove(&branch.name);
            } else {
                self.selected_branches.insert(branch.name.clone());
            }
        }
    }

    pub fn get_selected_branches(&self) -> Vec<String> {
        let mut branches: Vec<String> = self.selected_branches.iter().cloned().collect();
        branches.sort();
        branches
    }

    pub fn delete_selected_branches(&mut self) -> Result<()> {
        for branch_name in &self.selected_branches {
            git::delete_branch(&self.repo_path, branch_name)?;
        }

        self.reload_branches()?;
        Ok(())
    }

    pub fn is_protected_branch(&self, branch_name: &str) -> bool {
        if branch_name == "main" || branch_name == "master" {
            return true;
        }

        if let Some(branch) = self.branches.iter().find(|b| b.name == branch_name) {
            return branch.is_current;
        }

        false
    }
}