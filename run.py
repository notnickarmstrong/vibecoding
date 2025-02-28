import os
import time
import sys
import subprocess

INPUT_PROMPT = "<afk> hi! consult CLAUDE.md and keep going (: </afk>"
APPROVAL_MSG = "Claude what to do differently (esc)"
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
    
def main(workdir: str, session: str):
    p1 = p2 = ""
    while True: # ~ SOTA technology ~
        p1 = get_tmux_content(session)
        time.sleep(2)
        p2 = get_tmux_content(session)

        if is_finished(workdir):
            print("finished!")
            break
        if needs_approval(p1, p2):
            send_approval(session)
            print("sent approval")
            continue
        if needs_input(p1, p2):
            send_input(session)
            print("sent input")
            continue
        
def wrapper():
    session_default = "vibecoding"
    workdir_default = "project"
    session_prompt = f"tmux session name: (default: {session_default}): "
    workdir_prompt = f"working directory: (default: {workdir_default}): "
    session = input(session_prompt)
    workdir = input(workdir_prompt)
    
    # Use defaults if no input provided
    session = session_default if not session else session
    workdir = workdir_default if not workdir else workdir
    print(f"Using session: {session}")
    print(f"Using working directory: {workdir}")
    
    # Check if INSTRUCTIONS.md exists
    instructions_path = os.path.join(workdir, "INSTRUCTIONS.md") if os.path.exists(workdir) else "INSTRUCTIONS.md"
    
    if os.path.exists(instructions_path):
        # Read existing instructions
        with open(instructions_path, 'r') as f:
            instructions = f.read()
        print(f"Read existing instructions from {instructions_path}")
    else:
        # Prompt for instructions
        print("No existing INSTRUCTIONS.md found. Please provide instructions:")
        instructions = input("Instructions: ")
        print("Instructions saved.")
    
    # Create workdir if it doesn't exist
    if not os.path.exists(workdir):
        os.makedirs(workdir)
        print(f"Created working directory: {workdir}")
    
    # Save instructions to INSTRUCTIONS.md in workdir
    instructions_save_path = os.path.join(workdir, "INSTRUCTIONS.md")
    with open(instructions_save_path, 'w') as f:
        f.write(instructions)
    print(f"Saved instructions to {instructions_save_path}")
    
    # Save CLAUDE_MD to CLAUDE.md in workdir
    claude_md_path = os.path.join(workdir, "CLAUDE.md")
    with open("CLAUDE_MD", 'r') as src:
        with open(claude_md_path, 'w') as dst:
            dst.write(src.read())
    print(f"Copied CLAUDE_MD to {claude_md_path}")
    
    # Save INCOMPLETE_MD to INCOMPLETE.md in workdir
    incomplete_md_path = os.path.join(workdir, "INCOMPLETE.md")
    with open("INCOMPLETE_MD", 'r') as src:
        with open(incomplete_md_path, 'w') as dst:
            dst.write(src.read())
    print(f"Copied INCOMPLETE_MD to {incomplete_md_path}")
    
    print(f"Spawning claude in tmux session '{session}'...")
    import subprocess
    cmd = ["tmux", "new-session", "-d",
           "-s", session, "-c", workdir, "claude"]
    subprocess.run(cmd)
    print(f"Claude spawned in tmux session '{session}'")
    main(workdir, session)

if __name__ == "__main__":
    wrapper()
