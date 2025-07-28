# `tasksd`

This is the reference implementation for the 
[`taskscheduler`](https://github.com/joepigott/taskscheduler) server. The 
library implements most of the server functionality, and essentially only 
leaves the runtime and threading up to the server implementation. This 
reference uses `tokio`.

## Configuration

The following environment variables must be set:
* `TS_SERVER_ADDR`: The server address to use in the form `IPV4:PORT`.
* `TS_SCHEDULER_TIMEOUT`: The frequency the scheduling algorithm executes, in milliseconds.
* `TS_WRITE_TIMEOUT`: The frequency the program writes the data to disk, in minutes.
* `TS_STORAGE_PATH`: The path to store data when written to disk.

## API

For a comprehensive list of API endpoints, please see the 
[`taskscheduler` README](https://github.com/joepigott/taskscheduler/blob/master/README.md#api).
