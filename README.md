Task Keeper
=================
Task Keeper is a tool to manage tasks from different task file, such as `Makefile`, `package.json` etc.

# Features

* List tasks from different task files: `tk --list`
* Invoke task: `tk task_name`
* Invoke task from a runner: `tk --runner=npm start`
* Sync tasks between sources: `tk --from=npm --to=fleet task1 task2`

# Task runners to support

* Makefile(make): https://www.gnu.org/software/make/manual/make.html `make -qn`
* scripts in package.json(npm or yarn): https://docs.npmjs.com/cli/v8/using-npm/scripts
* tasks in deno.json(deno): https://deno.land/manual/tools/task_runner
* justfile(just): https://github.com/casey/just
* .fleet/run.json(fleet): https://www.jetbrains.com/help/fleet/run-configurations.html#reference
* .vscode/launch.json(code): https://code.visualstudio.com/docs/editor/debugging#_launchjson-attributes
* Jakefile(jake): https://jakejs.com/docs-page.html#item-overview-jakefiles
* Rakefile(rake): https://ruby.github.io/rake/
* Procfile(proc): https://devcenter.heroku.com/articles/procfile

