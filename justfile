rebase-all:
    #!/usr/bin/env nu
    print $"(ansi blue_bold)use ssh-agent(ansi reset)"
    ssh-add /home/long/.ssh/id_ed25519
    print $"(ansi blue_bold)jj git fetch --remote upstream(ansi reset)"
    jj git fetch --remote upstream
    print $"(ansi blue_bold)jj git fetch --remote upstream(ansi reset)"
    let bookmarks = [
        "main", # NOTE: includes pr/ipc-pointer-v2 et alia
        "pr/ipc-pointer-v2",
        "pr/match-output",
    ]
    $bookmarks | each {|bookmark|
        print $"(ansi blue_bold)jj rebase -d main@upstream -b ($bookmark) --ignore-immutable(ansi reset)"
        jj rebase -d main@upstream -b $bookmark --ignore-immutable
    }
    
force-push-all:
    #!/usr/bin/env nu
    print $"(ansi blue_bold)use ssh-agent(ansi reset)"
    ssh-add /home/long/.ssh/id_ed25519
    let bookmarks = [
        "main",
        # "pr/dont-expel-window-from-column-when-fullscreening", # NOTE: no willy-nilly ci/cd
        # "pr/workspace-focus-and-move-on-specified-output", # NOTE: no willy-nilly ci/cd
        "pr/ipc-pointer-v2",
        "pr/match-output",
        "pr/prioritize-last-output-and-workspace-config",
        "pr/no-focus-move-p2",
        "pr/activate-column",
    ]
    $bookmarks | each {|branch|
        print $"(ansi blue_bold)jj git push --bookmark ($branch) --remote origin(ansi reset)"
        jj git push --bookmark $branch --remote origin
    }
    

log:
    #!/usr/bin/env nu
    jj log -r main@upstream::


make-new-main-branch:
    #!/usr/bin/env nu
    let bookmarks = [
        "local",
        "pr/workspace-focus-and-move-on-specified-output",
        "pr/dont-expel-window-from-column-when-fullscreening",
        "pr/prioritize-last-output-and-workspace-config",
        "perf/surface-commit-win-refresh",
        "extern/svg-cursor",
        "extern/run-programs-directly",
        "extern/float-above-fullscreen",
        "pr/no-focus-move-p2",
        "pr/activate-column",
    ]
    let desc = ["feat: merge feature branches", "", ...$bookmarks] | str join "\n"
    jj new ...($bookmarks | each {["-r", $in]} | flatten)
    jj desc -m $desc
    jj bookmark move main --to @ --allow-backwards 
