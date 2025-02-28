

*instructions*

1. write your instructions: `echo "your instructions" > INSTRUCTIONS.md`
2. make a folder for claude to work in: `mkdir your-project-root`
3. move the needed files in: `cp CLAUDE.md INSTRUCTIONS.md INCOMPLETE.md your-project-root`
4. start claude in tmux in the folder: `cd your-project-root && tmux new-session -t vibecoding`
5. go through auth flow if needed and optionally get claude set up on your project
6. leave tmux by typing `<ctrl+b> <d>` (re-enter with `tmux a`)
7. navigate back here and run: `python run.py your-project-root vibecoding`


*safety*

in the absence of unprecedented global coordination on a short timescale, you are going to die.

have fun with auto-approved claude in the meantime.





