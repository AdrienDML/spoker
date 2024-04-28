set shell := ["nu", "-c"]
alias r := run
run TARGET: 
    cargo r --bin {{TARGET}}

alias rr := rerun 
rerun TARGET:
    loop {clear; try { cargo r --bin {{TARGET}}}; input}

