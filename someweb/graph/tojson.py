import json
import sys
nodes = eval(open(sys.argv[1]).read())
edges = eval(open(sys.argv[2]).read())
print(json.dumps({"nodes": nodes, "edges": edges}))
