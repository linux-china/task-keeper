# PowerShell completion for sq, add following line to your $PROFILE:
#   sq completion powershell | Out-String | Invoke-Expression
Register-ArgumentCompleter -Native -CommandName 'sq', 'sq.exe' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    # only complete the first argument: sub command or snippet name
    if ($commandAst.CommandElements.Count -gt 2 -or
        ($commandAst.CommandElements.Count -eq 2 -and $wordToComplete -eq '')) {
        return
    }

    $candidates = @('list', 'add', 'edit', 'completion', 'help')
    $snippetsFile = Join-Path $HOME '.tk/snippets.just'
    if (Test-Path $snippetsFile) {
        $summary = & just -f $snippetsFile --summary 2>$null
        if ($summary) {
            $candidates += ($summary -split '\s+' | Where-Object { $_ })
        }
    }

    $candidates | Where-Object { $_ -like "$wordToComplete*" } | Sort-Object -Unique | ForEach-Object {
        [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
    }
}
