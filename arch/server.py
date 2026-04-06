import http.server
import socketserver
import subprocess
import os
import urllib.parse
import json

PORT = 3000

class ZewpolServer(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        # Parse the URL to see if it's an app request
        parsed_path = urllib.parse.urlparse(self.path)
        
        if parsed_path.path == '/api/run-app':
            # Extract the query parameters (?app=terminal&cmd=...)
            params = urllib.parse.parse_qs(parsed_path.query)
            app_name = params.get('app', [''])[0]
            cmd_arg = params.get('cmd', [''])[0]
            
            file_path = f'./apps/{app_name}.py'
            
            if os.path.exists(file_path):
                try:
                    # Run the python file in your apps folder
                    if cmd_arg:
                        result = subprocess.check_output(['python3', file_path, cmd_arg], text=True, stderr=subprocess.STDOUT)
                    else:
                        result = subprocess.check_output(['python3', file_path], text=True, stderr=subprocess.STDOUT)
                    
                    output_data = {"output": result.strip()}
                except subprocess.CalledProcessError as e:
                    output_data = {"output": f"Error running app: {e.output}"}
            else:
                output_data = {"output": f"App file '{app_name}.py' not found in /apps."}

            # Send a successful JSON response back to the browser
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.send_header('Access-Control-Allow-Origin', '*')
            self.end_headers()
            self.wfile.write(json.dumps(output_data).encode())
            return
            
        # If it's not an API request, let the standard server serve index.html
        return super().do_GET()

# Run the server
with socketserver.TCPServer(("0.0.0.0", PORT), ZewpolServer) as httpd:
    print(f"ZewpolOS Python Bridge active on port {PORT}")
    httpd.serve_forever()