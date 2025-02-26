# `tasksd`

This is the reference implementation for the 
[`taskscheduler`](https://git.pigroy.xyz/pigroy/taskscheduler.git) server. The 
library implements most of the server functionality, and essentially only 
leaves the runtime and threading up to the server implementation. This 
reference uses `tokio`.

## API

For a comprehensive list of API endpoints, please see the 
[`taskscheduler` README](https://git.pigroy.xyz/pigroy/taskscheduler/src/branch/master/README.md)
