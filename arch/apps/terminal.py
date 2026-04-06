import sys
import subprocess
import json

def run():
    # Grab the command the user sent from the UI
    if len(sys.argv) > 1:
        command = sys.argv[1]
        try:
            # Run the physical bash command
            output = subprocess.check_output(command, shell=True, text=True, stderr=subprocess.STDOUT)
            print(json.dumps({"output": output}))
        except subprocess.CalledProcessError as e:
            print(json.dumps({"output": e.output}))
    else:
        print(json.dumps({"output": "No command received."}))

if __name__ == "__main__":
    run()