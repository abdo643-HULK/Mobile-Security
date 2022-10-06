use std::{
    io::{self, Write},
    process::{Command, Stdio},
};

fn main() {
    run_2_2();
}

fn run_2_2() {
    #[cfg(not(target_os = "linux"))]
    compile_error!("Your system isn't supported");

    #[cfg(target_os = "linux")]
    {
        let output = Command::new("bash")
            .args([
                "-c",
                "cat < /etc/passwd | grep $USER | cut -d':' -f1 > /tmp/username",
            ])
            .output()
            .expect("failed to execute process");

        if output.status.success() {
            // time writes to stderr instead of stderr
            println!("{:#?}", output);
            io::stdout().write_all(&output.stdout).unwrap();
        }

        let output = Command::new("bash")
            .args([
                "-c",
                r#"grep $(whoami) /etc/passwd | awk 'BEGIN { FS = ":" }; { print $1 }'"#,
            ])
            .output()
            .expect("failed to execute process");

        if output.status.success() {
            // time writes to stderr instead of stderr
            println!("{:#?}", output);
            io::stdout().write_all(&output.stdout).unwrap();
        }
    }
}

fn run_2_5() {
    let output = Command::new("bash")
        .args([
            "-c",
            "time find /Users/abdo/Downloads/ -type d >/dev/null 2>&1",
        ])
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        // time writes to stderr instead of stderr
        io::stdout().write_all(&output.stderr).unwrap();
    }
}
