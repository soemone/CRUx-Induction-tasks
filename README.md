# CRUx Induction tasks
### The repository that contains the tasks that have been required for me to complete in order to join the CRUx Coding club.
## Implementation details:
### VM-Calc
- This is currently an extension of a project that I was previously working on: https://github.com/soemone/vm-calc
- There have been many changes and this version could be considered an improvement over that implementation
- Notable changes to VM-Calc:
    - Implement haskell-like partial function calls, read the README.md of the folder to learn more
    - Implement arrays (currently a partial implementation and possibly a bad one as well), again, look through the respective README.md to learn more
    - You may now see the parsed output (Not as a tree, but as a formatted set of expressions that very much looks like the input) as well as the underlying instruction code
    - Currently, the REPL has not been re-implemented. So no REPL functionality at this time
    - Reduced the number of errors the parser handles, allowing the VM to handle the rest. A degradation in performance, perhaps.
    - Deleting items has not been tested at all. I presume some new code may interfere with it. I hope it does not.