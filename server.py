import http.server
import socketserver
import json
import urllib.parse


PORT = 8000
counter = 0

def actions(data):
  global counter
  print(json.dumps(data, indent=2))
  match data["kind"]:
    case "login" :
      print("login")
    case "cnt" :
      counter += 1
      print("cnt", counter)
    case _:
      print("err")
  return "hoge"

class MyHandler(http.server.SimpleHTTPRequestHandler):
  def do_GET(self):
    self.send_response(200)
    self.send_header("Content-type", "text/html; charset=utf-8")
    self.end_headers()
    html = """
    <!DOCTYPE html>
    <html>
    <head><title>Test Page</title></head>
    <body>
      <h1>Hello, world!</h1>
      <p>This is a simple HTML page.</p>
      <button id="postBtn">click</button>
    </body>
    <script>
      document.getElementById("postBtn").addEventListener("click", ()=> {
        window.ipc.postMessage(JSON.stringify({ kind : "count", data : 1 }));
      });
    </script>
    </html>
    """
    self.wfile.write(html.encode('utf-8'))
  def do_POST(self):
    content_len  = int(self.headers.get("content-length"))
    data = json.loads(self.rfile.read(content_len).decode('utf-8'))

    # url = urllib.parse.urlparse(fullpath)
    # match url.netloc:
    res = actions(data)
  
    self.send_response(200)
    self.send_header('Content-type', 'application/json;charset=utf-8')
    self.end_headers()
    body_json = json.dumps({ 'state' : True }, sort_keys=False, indent=4, ensure_ascii=False) 
    self.wfile.write(body_json.encode("utf-8"))
    self.end_headers()

with socketserver.TCPServer(("", PORT), MyHandler) as httpd:
  print("serving at port", PORT)
  httpd.serve_forever()