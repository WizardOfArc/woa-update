use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn template_dir() -> PathBuf {
    let woa_builder_repo_path_string =
        env::var("WOA_BUILDER_REPO").expect("WOA_BUILDER_REPO must be set");
    Path::new(woa_builder_repo_path_string.as_str()).to_path_buf()
}

pub fn commit_data_update(commit_message: &str) -> String {
    commit_update(commit_message, template_dir())
}

pub fn render_woa_templates() -> String {
    let template_dir_path_buff = template_dir();
    match env::set_current_dir(&template_dir_path_buff.as_path()) {
        Err(message) => format!("{:?} failed: {:?}", template_dir_path_buff, message),
        Ok(_) => {
            let output = Command::new("woa_site_builder.exe")
                .output()
                .expect("Site builder failed to run");
            String::from_utf8_lossy(&output.stdout).to_string()
        }
    }
}

pub fn commit_woa_update(commit_message: &str) -> String {
    let woa_dir_path_string: String =
        env::var("WOA_TARGET_DIR").expect("WOA_TARGET_DIR must be set");
    let woa_dir_path_buff = Path::new(woa_dir_path_string.as_str()).to_path_buf();
    commit_update(commit_message, woa_dir_path_buff)
}

pub fn commit_update(commit_message: &str, repo: PathBuf) -> String {
    match env::set_current_dir(&repo.as_path()) {
        Err(message) => format!("{:?} failed: {:?}", repo, message),
        Ok(_) => {
            let output1 = Command::new("git")
                .arg("commit")
                .arg("-am")
                .arg(format!("{:?}", commit_message))
                .output()
                .expect("commit failed");
            let output2 = Command::new("git")
                .arg("pull")
                .output()
                .expect("git pull failed"); // git commit -am "message"
            let output3 = Command::new("git")
                .arg("push")
                .output()
                .expect("git push failed");
            //let woa_repo_path_buff = git_repos.join("WizardOfArc.github.io");
            let outstr1 = String::from_utf8_lossy(&output1.stdout).to_string();
            let outstr2 = String::from_utf8_lossy(&output2.stdout).to_string();
            let outstr3 = String::from_utf8_lossy(&output3.stdout).to_string();
            format!("{:?} \n-> {:?} \n-> {:?}", outstr1, outstr2, outstr3)
        }
    }
}
