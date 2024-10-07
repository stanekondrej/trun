# trun - a task runner

trun is a task runner written in rust. it is very simple, minimal and easily configurable. a simple
yaml config file is all you need to get started, then just run `trun <path to config>`.

## the idea

the basic idea is that you have a file which contains tasks. these tasks have a name and a command.
trun goes through all tasks in your config file, and runs them. sounds familiar?

