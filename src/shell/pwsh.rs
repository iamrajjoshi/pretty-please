pub const INIT: &str = r#"function pls {
    param(
        [Parameter(ValueFromRemainingArguments = $true)]
        [string[]] $Args
    )

    $pls_bin = Get-Command pls -CommandType Application | Select-Object -First 1 -ExpandProperty Source
    if (-not $pls_bin) {
        Write-Error "pls binary not found in PATH."
        return
    }

    if ($Args.Count -eq 0) {
        $previous = (Get-History -Count 1).CommandLine
        if ([string]::IsNullOrWhiteSpace($previous)) {
            & $pls_bin --history-command $previous
            return
        }

        Invoke-Expression "& '$pls_bin' $previous"
    } else {
        & $pls_bin @Args
    }
}
"#;
