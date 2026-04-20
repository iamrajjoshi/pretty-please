pub const INIT: &str = r#"def --wrapped pls [...args: string] {
  if ($args | is-empty) {
    let previous = (history | last 1 | get command | first | default "")

    if (($previous | str trim) | is-empty) {
      ^pls --history-command $previous
    } else {
      nu -c $'^pls ($previous)'
    }
  } else {
    ^pls ...$args
  }
}
"#;
