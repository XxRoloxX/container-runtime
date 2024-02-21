/*
 * Simple server in Node.js
 */

// Import the http module
import http from "http";

// Create a server object
const server = http.createServer((req, res) => {
  // Set the response HTTP header with HTTP status and Content type
  res.writeHead(200, { "Content-Type": "text/plain" });
  console.log(`Request received from ${req.url}`);
  process.stdout.write("Request received\n");
  // Send the response body "Hello World"
  res.end("Hello from the container!\n");
});

// Prints a log once the server starts listening
server.listen(8080, () => {
  console.log("Server running at http://localhost:8080/");
});
