pub const INIT: &str = r#"function pls --description 'Re-run the previous command with sudo'
    if test (count $argv) -eq 0
        set -l previous (history --max=1)

        if test -z (string trim -- $previous)
            command pls --history-command "$previous"
            return $status
        end

        eval "command pls $previous"
    else
        command pls $argv
    end
end
"#;
