# trun - a task runner

trun is a task runner written in rust. it is very simple, minimal and easily configurable. a simple
yaml config file is all you need to get started, then just run `trun <path to config>`.

## usage

```bash
cd project-directory
trun # runs all tasks in the trun.yaml file
```

## the idea

the basic idea is that you have a file which contains tasks. these tasks have a name and a command.
trun goes through all tasks in your config file, and runs them. sounds familiar? my only problem
with makefiles is their complexity. this may be a little controversial, but the way that they are
written just doesn't suit me. that's why i created this. 

## the config files

take a look at the provided `trun.yaml` file. it doesn't get much simpler than that.

## to-do

- [ ] a mechanism for specifying dependencies in between tasks

