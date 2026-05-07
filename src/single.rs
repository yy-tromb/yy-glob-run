use ignore::overrides::OverrideBuilder;
use ignore::{WalkBuilder, WalkState};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio, exit};
use std::sync::Arc;

pub fn single_mode(
    (command, parallel, threads_number, _max_length, args): (
        String,
        bool,
        Option<usize>,
        usize,
        impl Iterator<Item = String>,
    ),
) {
    let args = args.collect::<Vec<_>>();
    let glob_info = args
        .iter()
        .enumerate()
        .filter(|(_, arg)| arg.contains('*') || arg.starts_with('!'))
        .collect::<Vec<_>>();

    // 2. バリデーション
    match glob_info.len() {
        0 => {
            crate::warn!("warn: this arguments do not have glob patterns.");
            execute_single_command(&command, &args);
            return;
        }
        1 => { /* 正常系: 続行 */ }
        _ => {
            crate::error!("Error: Multiple glob patterns are not supported.");
            exit(1);
        }
    }

    let (glob_index, glob_pattern) = glob_info
        .first()
        .map(|(index, pattern)| (*index, *pattern))
        .expect("Failed to info of 1st glob arg. This must not be happened.");

    // 3. ignore の準備
    let mut override_builder = OverrideBuilder::new("./");
    if let Err(e) = override_builder.add(glob_pattern) {
        crate::error!("Error: Invalid glob pattern '{}': {}", glob_pattern, e);
        exit(2);
    }
    let overrides = override_builder
        .build()
        .expect("Failed to build glob overrides.");

    // 共有用
    let cmd_arc = Arc::new(command);
    let args_arc = Arc::new(args);

    // 4. 実行ロジック
    // 各ファイルパスに対して、イテレータを回して引数を組み立てる
    let runner = move |path: &Path| {
        let path_str = path.strip_prefix("./").unwrap_or(path).to_string_lossy();

        let replaced_args = args_arc.iter().enumerate().map(|(i, arg)| {
            if i == glob_index {
                path_str.as_ref()
            } else {
                arg.as_str()
            }
        });

        let mut cmd = Command::new(&*cmd_arc);
        cmd.args(replaced_args); // イテレータをそのまま渡せる
        crate::info!("execute `{cmd:?}`");
        let mut child = match cmd.stdout(Stdio::inherit()).stderr(Stdio::piped()).spawn() {
            Ok(child) => child,
            Err(e) => {
                crate::error!("Error: {}\n`{cmd:?}`", e);
                exit(3);
            }
        };
        // 2. wait する前に stderr を読み切る
        if let Some(stderr) = child.stderr.take() {
            let reader = BufReader::new(stderr);
            reader
                .lines()
                .map_while(|result| result.ok())
                .for_each(|line| {
                    // 由来（ラベル）を付けて出力
                    eprintln!("[{:?}] {}", path, line);
                });
        }

        // 3. 全て読み終わったら終了を待つ
        let status = match child.wait() {
            Ok(s) => s,
            Err(e) => {
                crate::error!("Error: {}\n`{cmd:?}`", e);
                exit(4);
            }
        };
        if status.success() {
            crate::ok!("success `{cmd:?}`")
        } else {
            crate::error!(
                "Error: command exited with code {}\n`{cmd:?}`",
                status.code().unwrap_or(3),
            );
            exit(status.code().unwrap_or(3))
        }
    };

    // 5. WalkBuilder 構築と実行
    let mut builder = WalkBuilder::new("./");
    builder.overrides(overrides);
    if let Some(n) = threads_number {
        builder.threads(n);
    }

    if parallel {
        builder.build_parallel().run(|| {
            let runner_cloned = runner.clone();
            Box::new(move |result| {
                match result {
                    Ok(entry) => {
                        // ファイルであることの確認
                        if entry.file_type().is_some_and(|t| t.is_file()) {
                            runner_cloned(entry.path());
                        }
                    }
                    Err(e) => {
                        crate::error!("Error walking directory (parallel): {}", e);
                    }
                }
                WalkState::Continue
            })
        });
    } else {
        builder
            .build()
            .filter_map(|result| match result {
                Ok(entry) => Some(entry),
                Err(e) => {
                    crate::error!("Error walking directory: {}", e);
                    None
                }
            })
            .filter(|e| e.file_type().is_some_and(|t| t.is_file()))
            .for_each(|entry| runner(entry.path()));
    }
}

// globがない場合などの単発実行用ヘルパー
fn execute_single_command(cmd: &str, args: &[String]) {
    let status = Command::new(cmd).args(args).status();
    match status {
        Ok(s) if s.success() => {
            crate::ok!("success '{cmd}' with args: {args:?}")
        }
        Ok(s) => {
            crate::error!(
                "Error: command exited with code {}\n'{cmd}' with args: {args:?}",
                s.code().unwrap_or(3)
            );
            exit(s.code().unwrap_or(3))
        }
        Err(e) => {
            crate::error!("Error: {}\n'{cmd}' with args: {args:?}", e);
            exit(4);
        }
    }
}
