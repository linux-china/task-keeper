if (await Bun.file("Taskfile.ts").exists()) {
    let module = await import("./Taskfile.ts");
    if (Bun.argv.length >= 3) {
        let taskName = Bun.argv[2];
        let found = taskName in module;
        if (found) {
            let taskFunction = module[taskName];
            if (taskFunction["deps"]) {
                for (let dep of taskFunction["deps"]) {
                    await module[dep]();
                }
            }
            await taskFunction();
        } else {
            console.error(`Task not found: ${taskName}`);
        }

    } else {
        console.log("Available tasks:");
        Object.keys(module)
            .filter(taskName => typeof module[taskName] === "function")
            .forEach(taskName => {
                let taskFunction = module[taskName];
                let desc = taskFunction["desc"];
                if (desc) {
                    console.log(`* ${taskName} - ${desc}`)
                } else {
                    console.log(`* ${taskName}`)
                }
            });
    }
} else {
    console.error("Taskfile.ts not found");
}