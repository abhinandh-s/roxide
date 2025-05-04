
use builtin;
use str;

set edit:completion:arg-completer[roxide] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'roxide'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'roxide'= {
            cand -i 'whether to prompt before removals'
            cand --interactive 'whether to prompt before removals'
            cand -p 'remove files matching the pattern. revert will not work on patterns, provide -rp for recursive remove'
            cand --pattern 'remove files matching the pattern. revert will not work on patterns, provide -rp for recursive remove'
            cand -f 'Forces deletion without moving files to the trash directory'
            cand --force 'Forces deletion without moving files to the trash directory'
            cand -r 'Remove directories and their contents recursively'
            cand --recursive 'Remove directories and their contents recursively'
            cand -l 'list items which will be affected, (dry run)'
            cand --list 'list items which will be affected, (dry run)'
            cand -v 'Enable verbose output'
            cand --verbose 'Enable verbose output'
            cand -d 'remove empty directories'
            cand --dir 'remove empty directories'
            cand -c 'Will check health of roxide in user env'
            cand --check 'Will check health of roxide in user env'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
            cand revert 'revert the previous remove'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'roxide;revert'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'roxide;help'= {
            cand revert 'revert the previous remove'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'roxide;help;revert'= {
        }
        &'roxide;help;help'= {
        }
    ]
    $completions[$command]
}
