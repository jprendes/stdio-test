mod stdio;

pub(crate) mod sys {

#[cfg_attr(unix, path = "unix/stdio.rs")]
#[cfg_attr(windows, path = "windows/stdio.rs")]
pub mod stdio;

}

fn main() {
    let cwd = std::env::current_dir().unwrap();
    let pid = std::process::id();

    let stdio = stdio::Stdio {
        stdin: None::<String>.try_into().unwrap(),
        stdout: Some("./output.txt").try_into().unwrap(),
        stderr: None::<String>.try_into().unwrap(),
    };

    stdio.redirect().unwrap();

    println!("Hello, world!");
    println!("current dir = {cwd:?}");
    println!("pid = {pid:?}");
}
