import os
import time
import subprocess

INPUT_PROMPT = "<afk> hi! consult CLAUDE.md and keep going (: </afk>"
APPROVAL_MSG = "No, and tell Claude what to do differently (esc)"
CLAUDE_MD = """
# CODING AGENT INSTRUCTIONS
## DO NOT MODIFY
IMPORTANT: This instruction file must not be modified. You may edit any other files in the project, including README.md, but this file must remain unchanged.

## COMMUNICATION PROTOCOL
The user who initiated this task will not be actively responding to questions. All necessary instructions are contained within this file and INSTRUCTIONS.md. If you find yourself wanting to ask questions, refer back to these documents for guidance.

## PRIMARY OBJECTIVE
Your primary objective is defined in INSTRUCTIONS.md. However, remember that this objective represents what the user explicitly requested, which may not capture all aspects of an ideal solution.

## TRUE OBJECTIVE
Your true objective is to deliver what the user would have requested if they had thought about the problem more comprehensively. This means:

1. First, complete all explicitly stated requirements in INSTRUCTIONS.md
2. Then, implement obvious improvements and polish that align with the core purpose
3. Fix any clear design oversights in the original requirements
4. Ensure the solution is complete, robust, and user-friendly

## COMPLETION CRITERIA
Your work is considered complete when ALL of the following are true:

1. All explicit requirements specified in INSTRUCTIONS.md are fulfilled
2. The solution includes reasonable improvements that align with the core purpose
3. Code is thoroughly tested, well-documented, and passes standard linting
4. Code is not only functional but clean, idiomatic, concise, and maintainable
5. Project structure is logical, with clear entry points and documentation

When all criteria are met, you may remove the INCOMPLETE.md file from the project root to signal completion.

## DEVELOPMENT STANDARDS
When writing code, adhere to these principles:

1. Prioritize simplicity and readability over clever solutions
2. Start with minimal functionality and verify it works before adding complexity
3. Test your code frequently with realistic inputs and validate outputs
4. Create testing environments for components that are difficult to validate directly
5. Use functional and stateless approaches where they improve clarity
6. Keep core logic clean and push implementation details to the edges
7. Maintain consistent style (indentation, naming, patterns) throughout the codebase
8. Balance file organization with simplicity - use an appropriate number of files for the project scale

## PROJECT COMPLETION
You may delete INCOMPLETE.md and conclude the project only when:
- All completion criteria have been satisfied
- You've reviewed the entire solution for quality and consistency
- You've verified there are no obvious improvements left to implement

Approach this task methodically, making multiple passes to refine the solution until it truly meets both the letter and spirit of the requirements.
"""

INCOMPLETE_MD = """
# DO NOT REMOVE ME WITHOUT FOLLOWING THE INSTRUCTIONS IN CLAUDE.MD
"""


def send_tmux_keys(session: str, keys: str):
    cmd = ["tmux", "send-keys", "-t", session, keys]
    subprocess.run(cmd)

def get_tmux_content(session: str) -> str:
    cmd = ["tmux", "capture-pane", "-pt", session]
    result = subprocess.run(cmd, capture_output=True, text=True)
    return result.stdout

def send_approval(session: str):
    send_tmux_keys(session, "Down")
    send_tmux_keys(session, "C-m")

def send_input(session: str):
    keys = INPUT_PROMPT
    send_tmux_keys(session, keys)
    send_tmux_keys(session, "C-m")

def needs_approval(p1: str, p2: str) -> bool:
    approval_msg_in = lambda p: APPROVAL_MSG in p
    has_approval_msg = approval_msg_in(p1) and approval_msg_in(p2)
    return has_approval_msg and p1 == p2

def needs_input(p1: str, p2: str) -> bool:
    return p1 == p2

def is_finished(workdir: str) -> bool:
    return not os.path.exists(
        os.path.join(workdir, "INCOMPLETE.md")
    )
    
def manage_claude(session: str, workdir: str,
                  autoapprove=True,
                  autocontinue=False,
                  check_finished=False):
    p1 = p2 = ""
    while True: # ~ SOTA technology ~
        p1 = get_tmux_content(session)
        time.sleep(2)
        p2 = get_tmux_content(session)

        if check_finished and is_finished(workdir):
            print("finished!")
            break
        if autoapprove and needs_approval(p1, p2):
            send_approval(session)
            print("sent approval")
            continue
        if autocontinue and needs_input(p1, p2):
            send_input(session)
            print("sent input")
            continue

def tmux_session_exists(session: str):
    try:
        subprocess.check_output(
                ['tmux', 'has-session', '-t', session], 
                stderr=subprocess.DEVNULL
        )
        return True
    except subprocess.CalledProcessError:
        return False

def spawn_claude(session: str, workdir: str):
    print(f"spawning claude in tmux session '{session}'...")
    try:
        cmd = ["tmux", "new-session", "-d",
               "-s", session, "-c", workdir, "claude"]
        subprocess.run(cmd)
        if not tmux_session_exists(session):
            raise Exception
    except Exception:
        print(f"error making tmux for claude... make sure they're on path")
        exit(1)
    print(f"claude is clauding...\n")
    print(f"(to watch, run \"tmux a -t {session}\")\n\n")

def get_instructions_path(workdir: str) -> str:
    return os.path.join(workdir, "INSTRUCTIONS.md")

def get_claude_md_path(workdir: str) -> str:
    return os.path.join(workdir, "CLAUDE.md")

def get_finish_flag_path(workdir: str) -> str:
    return os.path.join(workdir, "INCOMPLETE.md")

def ask_user_for_session_name(default="vibecoding") -> str:
    session_prompt = f"(unique) project name: (default: {default}): "
    session_input = input(session_prompt)
    return session_input if session_input else default

def ask_and_save_user_instructions_to(path_to_save_to: str):
    print('what are we vibecoding today?\n')
    instructions_content = input("> ")
    if not instructions_content:
        print('claude needs something to do! exiting...')
        exit(1)
    
    with open(path_to_save_to, 'w') as dst:
        dst.write(instructions_content)
        print(f'saved instructions to {path_to_save_to}')

def ensure_workdir(workdir: str):
    if not os.path.exists(workdir):
        os.makedirs(workdir)
        print(f'created project root {workdir}')

def ensure_claude_md(workdir: str):
    path = get_claude_md_path(workdir)
    if not os.path.exists(path):
        with open(path, 'w') as dst:
            dst.write(CLAUDE_MD)
        print(f'created {path}')

def ensure_instructions(workdir: str):
    path = get_instructions_path(workdir)
    if not os.path.exists(path):
        ask_and_save_user_instructions_to(path)

def ensure_finish_flag(workdir: str):
    path = get_finish_flag_path(workdir)
    if not os.path.exists(path):
        with open(path, 'w') as dst:
            dst.write(INCOMPLETE_MD)
        print(f'created {path}')

def run(session=None,
        workdir=None,
        autoapprove=True,
        autocontinue=True,
        check_finished=True):
    
    # we always need a tmux session
    if session is None:
        session = ask_user_for_session_name()
    
    # set the workdir to session name if missing
    workdir = workdir if workdir else session

    # till im confident argparse logic is right
    print(f'session={session}')
    print(f'workdir={workdir}')
    print(f'autoapprove={autoapprove}')
    print(f'autocontinue={autocontinue}')
    print(f'check_finished={check_finished}')
    print(f'\n\n')

    # make sure claude has the files it needs
    ensure_workdir(workdir)
    ensure_claude_md(workdir)
    ensure_instructions(workdir)
    ensure_finish_flag(workdir) if check_finished else ...

    # spawn claude in a session if needed
    if not tmux_session_exists(session):
        spawn_claude(session, workdir)
    
    # do the thing
    manage_claude(session, workdir,
                  autoapprove=autoapprove,
                  autocontinue=autocontinue,
                  check_finished=check_finished)

def init(workdir: str, check_finished=True):
    ensure_workdir(workdir)
    ensure_claude_md(workdir)
    ensure_finish_flag(workdir) if check_finished else ...
    instruction_path = get_instructions_path(workdir)
    if not os.path.exists(instruction_path):
        with open(instruction_path, 'w') as dst:
            dst.write("put your instructions here")

    print(f'set up a new vibecoder project: {workdir}')
    print(f'fill in {instruction_path}', end=' ')
    print(f'and run \'python vibecode.py {workdir}\'')

if __name__ == "__main__":
    import argparse
    import sys
    
    # Check for help command first
    if len(sys.argv) > 1 and sys.argv[1] == "help":
        print("Usage:")
        print("  python vibecode.py init <workdir> [--check-finished=bool]")
        print("  python vibecode.py autoapprove <session>")
        print("  python vibecode.py [session [workdir]] [--flags]")
        print("\nFlags:")
        print("  --autoapprove=bool    Default: True")
        print("  --autocontinue=bool   Default: True")
        print("  --check-finished=bool Default: True")
        print("  --session=string      Session name")
        print("  --workdir=string      Working directory")
        sys.exit(0)
    
    # Create parser
    parser = argparse.ArgumentParser(add_help=False)
    
    # Add the command (init or run) - optional
    parser.add_argument('command', nargs='?', default=None)
    
    # Add positional arguments - session and workdir
    parser.add_argument('arg1', nargs='?', default=None)
    parser.add_argument('arg2', nargs='?', default=None)
    
    # Add flags
    parser.add_argument('--autoapprove', type=lambda x: x.lower() == 'true', default=True)
    parser.add_argument('--autocontinue', type=lambda x: x.lower() == 'true', default=True)
    parser.add_argument('--check-finished', type=lambda x: x.lower() == 'true', default=True)
    parser.add_argument('--session', type=str, default=None)
    parser.add_argument('--workdir', type=str, default=None)
    
    args = parser.parse_args()
    
    # Handle the init command
    if args.command == "init":
        workdir = args.arg1
        if workdir is None:
            print("Error: init command requires a workdir argument")
            sys.exit(1)
        init(workdir, check_finished=args.check_finished)
    
    # Handle the autoapprove command
    elif args.command == "autoapprove":
        session = args.arg1
        if session is None:
            print("Error: autoapprove command requires a session argument")
            sys.exit(1)
        # Call run with autoapprove=True and the specified session
        run(session=session, autoapprove=True, autocontinue=False, check_finished=False)
    
    # Handle the run command (default)
    else:
        # If command is not 'init' or 'autoapprove', it might be a session name
        session = args.session or args.command
        
        # Determine the workdir based on arguments and flags
        if args.workdir:
            # If workdir flag is provided, use it directly
            workdir = args.workdir
        elif args.command is not None and args.arg1 is not None:
            # If both command and arg1 are provided, arg1 is the workdir
            workdir = args.arg1
        elif args.command is not None and args.session is not None and args.arg1 is not None:
            # If command is used as session name and --session flag is also provided with arg1
            workdir = args.arg1
        elif args.arg2 is not None:
            # If arg2 is provided, it's the workdir
            workdir = args.arg2
        else:
            # Default to using the session name as workdir
            workdir = session







