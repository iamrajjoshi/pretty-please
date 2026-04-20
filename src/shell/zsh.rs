pub const INIT: &str = r#"pls() {
  if [ "$#" -eq 0 ]; then
    local previous
    previous="$(fc -ln -1 2>/dev/null)" || {
      command pls --history-command ""
      return $?
    }

    if [[ "$previous" != *[![:space:]]* ]]; then
      command pls --history-command "$previous"
      return $?
    fi

    builtin eval "command pls ${previous}"
  else
    command pls "$@"
  fi
}
"#;
