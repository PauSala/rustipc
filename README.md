# rustipc

## Inter Process Communication in Rust (IPC)

Example of IPC in Rust using interprocess crate.
There are two programs, a server and a client.
Communication is done using local sockets.
The server calls a command opening the client program. It also listens for messages from the client.
The client is a simple dixous app that exposes two buttons to send a hello message and a stop message to the server.
When a stop message is send, the client coses the window and the server stops listening, then the recieved messages are processed and the program stops.

## Purpose
Proof of concept of a multiprocess architecture for opening a dummy UI from a main process and communicate the user input through local sockets.
