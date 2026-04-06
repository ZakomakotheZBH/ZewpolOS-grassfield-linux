import os
import json

def run():
    # Read the main directory
    files = os.listdir('/workspaces/linux')
    # Return them as a clean JSON list for the UI
    print(json.dumps({"files": files}))

if __name__ == "__main__":
    run()