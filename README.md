Task Keeper
=================
tk(Task Keeper) is a tool to manage tasks from different task file,
such as `Makefile`,`justfile`, `package.json`, `deno.jso`, `.fleet/run.json` etc.

![Task Keeper](./screenshot.png)

# Features

* List tasks from different task files: `tk --list`
* Invoke task: `tk task_name`
* Invoke task from a runner: `tk --runner=npm start`
* Sync tasks between sources: `tk --from=npm --to=fleet task1 task2`
* .env support by default: `tk --no-dotenv` to disable

# Task runners support

* make(Makefile): https://www.gnu.org/software/make/manual/make.html `make -qn`
* npm(package.json): https://docs.npmjs.com/cli/v8/using-npm/scripts
* deno(deno.json): https://deno.land/manual/tools/task_runner
* just(justfile): https://github.com/casey/just
* fleet(fleet/run.json): https://www.jetbrains.com/help/fleet/run-configurations.html#reference
* Rakefile(rake): https://ruby.github.io/rake/
* invoke(tasks.py): https://www.pyinvoke.org/
* task(Taskfile.yml): https://github.com/go-task/task  https://www.sobyte.net/post/2022-04/taskfile/
* cargo-make(Makefile.toml):  https://github.com/sagiegurari/cargo-make
* proc(Procfile): https://devcenter.heroku.com/articles/procfile
* markdown(README.md): shell code block support

# How to get started?

* Install by cargo: `cargo install task-keeper`
* Download and install from [GitHub Releases](https://github.com/linux-china/task-keeper/releases)

After install, execute `tk --help` for usage. Some commands as following: 

* list tasks: `tk --list`
* execute task: `tk task_name`

# Todo

* jake(Jakefile): https://jakejs.com/docs-page.html#item-overview-jakefiles
* code(.vscode/launch.json): https://code.visualstudio.com/docs/editor/debugging#_launchjson-attributes

# Shell scripts in Markdown

```shell
$ ## http-methods
$ curl https://httpbin.org/get
$ curl -X POST https://httpbin.org/post
```

```shell
## print my internet ip
curl --silent https://httpbin.org/ip | jq '.origin'
```

```shell
curl https://httpbin.org/ip \
    --user-agent "Task Keeper/0.1.0" \
    --silent
curl https://httpbin.org/headers
```
