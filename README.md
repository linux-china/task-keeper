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

# Todo

* jake(Jakefile): https://jakejs.com/docs-page.html#item-overview-jakefiles
* code(.vscode/launch.json): https://code.visualstudio.com/docs/editor/debugging#_launchjson-attributes

# Shell scripts in Markdown

```shell
$ ## http-methods
$ curl https://httpsbin.org/get
$ curl https://httpsbin.org/post
```

```shell
## print my internet ip
curl https://httpsbin.org/ip
```

```shell
curl https://httpbin.org/ip \
    | grep 'origin' \
    | head -n 1
curl https://httpbin.org/headers
```
